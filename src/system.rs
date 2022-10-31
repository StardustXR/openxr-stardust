use crate::{util::copy_str_to_buffer, XrResult};
use openxr_sys::{
	FormFactor, Instance, StructureType, SystemGetInfo, SystemId, SystemProperties,
	ViewConfigurationProperties, ViewConfigurationType, ViewConfigurationView,
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

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateViewConfigurations
#[no_mangle]
pub unsafe extern "system" fn xrEnumerateViewConfigurations(
	_instance: Instance,
	_system_id: SystemId,
	_view_configuration_type_capacity_input: u32,
	_view_configuration_type_count_output: &mut u32,
	_view_configuration_types: &mut ViewConfigurationType,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetViewConfigurationProperties
#[no_mangle]
pub unsafe extern "system" fn xrGetViewConfigurationProperties(
	_instance: Instance,
	_system_id: SystemId,
	_view_configuration_type: ViewConfigurationType,
	_configuration_properties: &mut ViewConfigurationProperties,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateViewConfigurationViews
#[no_mangle]
pub unsafe extern "system" fn xrEnumerateViewConfigurationViews(
	_instance: Instance,
	_system_id: SystemId,
	_view_configuration_type: ViewConfigurationType,
	_view_capacity_input: u32,
	_view_count_output: &mut u32,
	_views: &mut ViewConfigurationView,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}
