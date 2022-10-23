use crate::{oxr::ExtensionProperties, util::enumerate};
use std::ffi::c_char;

/// # Safety
/// https://registry.khronos.org/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateInstanceExtensionProperties
#[no_mangle]
pub unsafe extern "system" fn xrEnumerateInstanceExtensionProperties(
	_layer_name: *const c_char,
	input_count: u32,
	output_count: &mut Option<u32>,
	items_ptr: *mut ExtensionProperties,
) -> openxr_sys::Result {
	let extensions = [];
	enumerate(input_count, output_count, items_ptr, &extensions)
}
