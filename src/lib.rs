#[macro_use]
pub mod util;
pub mod extensions;
pub mod instance;
pub mod session;
mod string;
pub mod system;
pub mod wip;

pub use openxr_sys as oxr;

use extensions::xrEnumerateInstanceExtensionProperties;
use instance::{xrCreateInstance, StardustInstance};
use oxr::{
	loader::{XrNegotiateLoaderInfo, XrNegotiateRuntimeRequest, CURRENT_LOADER_RUNTIME_VERSION},
	pfn::VoidFunction,
	ApiLayerProperties, Instance, CURRENT_API_VERSION,
};
use std::{
	ffi::c_char,
	mem::{size_of, transmute},
};
use util::{enumerate, str_from_const_char};

pub type XrResult = openxr_sys::Result;

/// https://registry.khronos.org/OpenXR/specs/1.0/html/xrspec.html#xrNegotiateLoaderRuntimeInterface
#[no_mangle]
pub extern "system" fn xrNegotiateLoaderRuntimeInterface(
	loader_info: &XrNegotiateLoaderInfo,
	runtime_request: &mut XrNegotiateRuntimeRequest,
) -> XrResult {
	if loader_info.ty != XrNegotiateLoaderInfo::TYPE
		|| loader_info.struct_version != XrNegotiateLoaderInfo::VERSION
		|| loader_info.struct_size != size_of::<XrNegotiateLoaderInfo>()
	{
		return XrResult::ERROR_INITIALIZATION_FAILED;
	}
	if runtime_request.ty != XrNegotiateRuntimeRequest::TYPE
		|| runtime_request.struct_version != XrNegotiateRuntimeRequest::VERSION
		|| loader_info.struct_size != size_of::<XrNegotiateRuntimeRequest>()
	{
		return XrResult::ERROR_INITIALIZATION_FAILED;
	}

	if CURRENT_API_VERSION > loader_info.max_api_version
		|| CURRENT_API_VERSION < loader_info.min_api_version
	{
		eprintln!(
			"OpenXR Runtime doesn't support major version {} < {} < {}",
			loader_info.max_api_version, CURRENT_API_VERSION, loader_info.min_api_version
		);
		return XrResult::ERROR_INITIALIZATION_FAILED;
	}

	runtime_request.runtime_interface_version = CURRENT_LOADER_RUNTIME_VERSION;
	runtime_request.runtime_api_version = CURRENT_API_VERSION;
	runtime_request.get_instance_proc_addr =
		Some(unsafe { transmute(xrGetInstanceProcAddr as usize) });

	XrResult::SUCCESS
}

/// # Safety
/// https://registry.khronos.org/OpenXR/specs/1.0/html/xrspec.html#xrGetInstanceProcAddr
#[no_mangle]
pub unsafe extern "system" fn xrGetInstanceProcAddr(
	instance: Instance,
	name: *const c_char,
	function: &mut VoidFunction,
) -> XrResult {
	wrap_oxr! {
		let instance = if instance.into_raw() == 0_u64 {
			None
		} else {
			Some(instance)
		};
		*function = get_instance_proc_addr(instance, str_from_const_char(name)?)?;
	}
}

fn get_instance_proc_addr(
	instance: Option<Instance>,
	name: &str,
) -> Result<VoidFunction, XrResult> {
	match instance {
		None => oxr_fns![
			name,
			xrEnumerateInstanceExtensionProperties,
			xrEnumerateApiLayerProperties,
			xrCreateInstance
		],
		Some(instance) => StardustInstance::from_oxr(instance)?.get_proc_addr(name),
	}
}

/// # Safety
/// https://registry.khronos.org/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateApiLayerProperties
#[no_mangle]
pub unsafe extern "system" fn xrEnumerateApiLayerProperties(
	property_capacity_input: u32,
	property_count_output: &mut Option<u32>,
	properties: *mut ApiLayerProperties,
) -> XrResult {
	wrap_oxr! {
		let api_layers = [];
		enumerate(
			property_capacity_input,
			property_count_output,
			properties,
			&api_layers,
		)?;
	}
}
