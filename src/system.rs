use crate::{util::copy_str_to_buffer, XrResult};
use openxr_sys::{
	FormFactor, Instance, StructureType, SystemGetInfo, SystemId, SystemProperties,
	MIN_COMPOSITION_LAYERS_SUPPORTED, TRUE,
};

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetSystem
#[no_mangle]
pub extern "system" fn xrGetSystem(
	_instance: Instance,
	get_info: &SystemGetInfo,
	system_id: &mut SystemId,
) -> XrResult {
	if get_info.form_factor == FormFactor::HEAD_MOUNTED_DISPLAY {
		*system_id = SystemId::from_raw(get_info.form_factor.into_raw() as u64);
		XrResult::SUCCESS
	} else {
		XrResult::ERROR_FORM_FACTOR_UNSUPPORTED
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetSystemProperties
#[no_mangle]
pub unsafe extern "system" fn xrGetSystemProperties(
	_instance: Instance,
	system_id: SystemId,
	properties: &mut SystemProperties,
) -> XrResult {
	wrap_oxr! {
		properties.ty = StructureType::SYSTEM_PROPERTIES;
		properties.system_id = system_id;
		copy_str_to_buffer("Stardust Virtual Device", &mut properties.system_name);
		properties.tracking_properties.orientation_tracking = TRUE;
		properties.tracking_properties.position_tracking = TRUE;
		properties.graphics_properties.max_layer_count = MIN_COMPOSITION_LAYERS_SUPPORTED as u32;
		properties.graphics_properties.max_swapchain_image_width = 1024 * 16;
		properties.graphics_properties.max_swapchain_image_height = 1024 * 16;

		Ok(())
	}
}
