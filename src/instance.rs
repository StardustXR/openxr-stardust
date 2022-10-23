use crate::{
	extensions::xrEnumerateInstanceExtensionProperties,
	oxr::{pfn::VoidFunction, Instance, InstanceCreateInfo, Version},
	string::{xrResultToString, xrStructureTypeToString},
	util::string_from_const_char,
	xrEnumerateApiLayerProperties, XrResult,
};

#[derive(Debug)]
pub struct StardustInstance {
	app_name: String,
	app_version: u32,
	engine_name: String,
	engine_version: u32,
	api_version: Version,
}
impl StardustInstance {
	pub fn from_oxr<'a>(instance: Instance) -> &'a mut StardustInstance {
		unsafe { &mut *(instance.into_raw() as *mut StardustInstance) }
	}
	pub fn get_proc_addr(&self, name: &str) -> Result<VoidFunction, XrResult> {
		oxr_fns![
			name,
			xrEnumerateInstanceExtensionProperties,
			xrEnumerateApiLayerProperties,
			xrCreateInstance,
			xrDestroyInstance,
			xrResultToString,
			xrStructureTypeToString
		]
	}
}

/// # Safety
/// https://registry.khronos.org/OpenXR/specs/1.0/html/xrspec.html#xrCreateInstance
#[no_mangle]
pub unsafe extern "system" fn xrCreateInstance(
	create_info: &InstanceCreateInfo,
	instance: &mut Instance,
) -> XrResult {
	let stardust_instance = Box::new(StardustInstance {
		app_name: string_from_const_char(create_info.application_info.application_name.as_ptr())
			.unwrap(),
		app_version: create_info.application_info.application_version,
		engine_name: string_from_const_char(create_info.application_info.engine_name.as_ptr())
			.unwrap(),
		engine_version: create_info.application_info.engine_version,
		api_version: create_info.application_info.api_version,
	});
	dbg!(&stardust_instance);
	*instance = Instance::from_raw(Box::into_raw(stardust_instance) as u64);

	XrResult::SUCCESS
}

/// # Safety
/// https://registry.khronos.org/OpenXR/specs/1.0/html/xrspec.html#xrDestroyInstance
#[no_mangle]
pub unsafe extern "system" fn xrDestroyInstance(instance: Instance) -> XrResult {
	if instance.into_raw() == 0 {
		return XrResult::ERROR_HANDLE_INVALID;
	}

	drop(Box::from_raw(StardustInstance::from_oxr(instance) as *mut _));

	XrResult::SUCCESS
}
