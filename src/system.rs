use std::ptr;

use crate::{
	instance::StardustInstance,
	util::{copy_str_to_buffer, enumerate},
	XrResult,
};
use openxr_sys::{
	EnvironmentBlendMode, Instance, StructureType, SystemGetInfo, SystemId, SystemProperties,
	ViewConfigurationProperties, ViewConfigurationType, ViewConfigurationView,
	MIN_COMPOSITION_LAYERS_SUPPORTED, TRUE,
};
use serde::Deserialize;

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetSystem
#[no_mangle]
pub extern "system" fn xrGetSystem(
	instance: Instance,
	get_info: &SystemGetInfo,
	system_id: &mut SystemId,
) -> XrResult {
	wrap_oxr! {
		let instance = StardustInstance::from_oxr(instance)?;
		let system = instance.execute_method("/openxr", "get_system", &(get_info.form_factor.into_raw() as u32))?;
		let system_type: u32 = system.map_err(|_| XrResult::ERROR_FORM_FACTOR_UNSUPPORTED)?;
		*system_id = SystemId::from_raw(system_type as u64);
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
	instance: Instance,
	system_id: SystemId,
	view_configuration_type: ViewConfigurationType,
	view_capacity_input: u32,
	view_count_output: &mut Option<u32>,
	views_ptr: *mut ViewConfigurationView,
) -> XrResult {
	#[derive(Debug, Deserialize)]
	struct StardustView {
		recommended_image_rect_width: u32,
		max_image_rect_width: u32,
		recommended_image_rect_height: u32,
		max_image_rect_height: u32,
	}
	wrap_oxr! {
		let stardust_instance = StardustInstance::from_oxr(instance)?;
		let views: Vec<StardustView> = stardust_instance.execute_method(&format!("/openxr/system{}", system_id.into_raw()), "views", &view_configuration_type.into_raw())?.map_err(|_| XrResult::ERROR_HANDLE_INVALID)?;
		let views = views.into_iter().map(|v| ViewConfigurationView {
			ty: StructureType::VIEW_CONFIGURATION_VIEW,
			next: ptr::null_mut(),
			recommended_image_rect_width: v.recommended_image_rect_width,
			max_image_rect_width: v.max_image_rect_width,
			recommended_image_rect_height: v.recommended_image_rect_height,
			max_image_rect_height: v.max_image_rect_height,
			recommended_swapchain_sample_count: 1,
			max_swapchain_sample_count: 1
		}).collect::<Vec<_>>();

		enumerate(view_capacity_input, view_count_output, views_ptr, &views)?;
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateEnvironmentBlendModes
#[no_mangle]
pub unsafe extern "system" fn xrEnumerateEnvironmentBlendModes(
	_instance: Instance,
	_system_id: SystemId,
	_view_configuration_type: ViewConfigurationType,
	_environment_blend_mode_capacity_input: u32,
	_environment_blend_mode_count_output: &mut u32,
	_environment_blend_modes: &mut EnvironmentBlendMode,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}
