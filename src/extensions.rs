use openxr_sys::MND_HEADLESS_EXTENSION_NAME;

use crate::{
	oxr::ExtensionProperties,
	util::{copy_str_to_buffer, enumerate},
};
use std::{ffi::c_char, ptr};

/// # Safety
/// https://registry.khronos.org/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateInstanceExtensionProperties
#[no_mangle]
pub unsafe extern "system" fn xrEnumerateInstanceExtensionProperties(
	_layer_name: *const c_char,
	input_count: u32,
	output_count: &mut Option<u32>,
	items_ptr: *mut ExtensionProperties,
) -> openxr_sys::Result {
	wrap_oxr! {
		let mut headless = ExtensionProperties { ty: ExtensionProperties::TYPE, next: ptr::null_mut(), extension_name: [0; 128], extension_version: 2 };
		copy_str_to_buffer(&String::from_utf8(MND_HEADLESS_EXTENSION_NAME.to_vec()).unwrap(), &mut headless.extension_name);
		let extensions = [
			headless
		];
		enumerate(input_count, output_count, items_ptr, &extensions)?;
	}
}
