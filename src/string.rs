use openxr_sys::Path;
use slotmap::Key;

use crate::{
	oxr::{Instance, StructureType, MAX_RESULT_STRING_SIZE, MAX_STRUCTURE_NAME_SIZE},
	util::{enumerate, str_from_const_char, Handle},
	XrResult,
};
use std::ffi::c_char;

/// # Safety
/// https://registry.khronos.org/OpenXR/specs/1.0/html/xrspec.html#xrResultToString
#[no_mangle]
pub unsafe extern "system" fn xrResultToString(
	_instance: Instance,
	value: XrResult,
	buffer: *mut c_char,
) -> XrResult {
	let string = match value {
		XrResult::SUCCESS => Some("SUCCESS"),
		XrResult::TIMEOUT_EXPIRED => Some("TIMEOUT_EXPIRED"),
		XrResult::SESSION_LOSS_PENDING => Some("SESSION_LOSS_PENDING"),
		XrResult::EVENT_UNAVAILABLE => Some("EVENT_UNAVAILABLE"),
		XrResult::SPACE_BOUNDS_UNAVAILABLE => Some("SPACE_BOUNDS_UNAVAILABLE"),
		XrResult::SESSION_NOT_FOCUSED => Some("SESSION_NOT_FOCUSED"),
		XrResult::FRAME_DISCARDED => Some("FRAME_DISCARDED"),
		XrResult::ERROR_VALIDATION_FAILURE => Some("ERROR_VALIDATION_FAILURE"),
		XrResult::ERROR_RUNTIME_FAILURE => Some("ERROR_RUNTIME_FAILURE"),
		XrResult::ERROR_OUT_OF_MEMORY => Some("ERROR_OUT_OF_MEMORY"),
		XrResult::ERROR_API_VERSION_UNSUPPORTED => Some("ERROR_API_VERSION_UNSUPPORTED"),
		XrResult::ERROR_INITIALIZATION_FAILED => Some("ERROR_INITIALIZATION_FAILED"),
		XrResult::ERROR_FUNCTION_UNSUPPORTED => Some("ERROR_FUNCTION_UNSUPPORTED"),
		XrResult::ERROR_FEATURE_UNSUPPORTED => Some("ERROR_FEATURE_UNSUPPORTED"),
		XrResult::ERROR_EXTENSION_NOT_PRESENT => Some("ERROR_EXTENSION_NOT_PRESENT"),
		XrResult::ERROR_LIMIT_REACHED => Some("ERROR_LIMIT_REACHED"),
		XrResult::ERROR_SIZE_INSUFFICIENT => Some("ERROR_SIZE_INSUFFICIENT"),
		XrResult::ERROR_HANDLE_INVALID => Some("ERROR_HANDLE_INVALID"),
		XrResult::ERROR_INSTANCE_LOST => Some("ERROR_INSTANCE_LOST"),
		XrResult::ERROR_SESSION_RUNNING => Some("ERROR_SESSION_RUNNING"),
		XrResult::ERROR_SESSION_NOT_RUNNING => Some("ERROR_SESSION_NOT_RUNNING"),
		XrResult::ERROR_SESSION_LOST => Some("ERROR_SESSION_LOST"),
		XrResult::ERROR_SYSTEM_INVALID => Some("ERROR_SYSTEM_INVALID"),
		XrResult::ERROR_PATH_INVALID => Some("ERROR_PATH_INVALID"),
		XrResult::ERROR_PATH_COUNT_EXCEEDED => Some("ERROR_PATH_COUNT_EXCEEDED"),
		XrResult::ERROR_PATH_FORMAT_INVALID => Some("ERROR_PATH_FORMAT_INVALID"),
		XrResult::ERROR_PATH_UNSUPPORTED => Some("ERROR_PATH_UNSUPPORTED"),
		XrResult::ERROR_LAYER_INVALID => Some("ERROR_LAYER_INVALID"),
		XrResult::ERROR_LAYER_LIMIT_EXCEEDED => Some("ERROR_LAYER_LIMIT_EXCEEDED"),
		XrResult::ERROR_SWAPCHAIN_RECT_INVALID => Some("ERROR_SWAPCHAIN_RECT_INVALID"),
		XrResult::ERROR_SWAPCHAIN_FORMAT_UNSUPPORTED => Some("ERROR_SWAPCHAIN_FORMAT_UNSUPPORTED"),
		XrResult::ERROR_ACTION_TYPE_MISMATCH => Some("ERROR_ACTION_TYPE_MISMATCH"),
		XrResult::ERROR_SESSION_NOT_READY => Some("ERROR_SESSION_NOT_READY"),
		XrResult::ERROR_SESSION_NOT_STOPPING => Some("ERROR_SESSION_NOT_STOPPING"),
		XrResult::ERROR_TIME_INVALID => Some("ERROR_TIME_INVALID"),
		XrResult::ERROR_REFERENCE_SPACE_UNSUPPORTED => Some("ERROR_REFERENCE_SPACE_UNSUPPORTED"),
		XrResult::ERROR_FILE_ACCESS_ERROR => Some("ERROR_FILE_ACCESS_ERROR"),
		XrResult::ERROR_FILE_CONTENTS_INVALID => Some("ERROR_FILE_CONTENTS_INVALID"),
		XrResult::ERROR_FORM_FACTOR_UNSUPPORTED => Some("ERROR_FORM_FACTOR_UNSUPPORTED"),
		XrResult::ERROR_FORM_FACTOR_UNAVAILABLE => Some("ERROR_FORM_FACTOR_UNAVAILABLE"),
		XrResult::ERROR_API_LAYER_NOT_PRESENT => Some("ERROR_API_LAYER_NOT_PRESENT"),
		XrResult::ERROR_CALL_ORDER_INVALID => Some("ERROR_CALL_ORDER_INVALID"),
		XrResult::ERROR_GRAPHICS_DEVICE_INVALID => Some("ERROR_GRAPHICS_DEVICE_INVALID"),
		XrResult::ERROR_POSE_INVALID => Some("ERROR_POSE_INVALID"),
		XrResult::ERROR_INDEX_OUT_OF_RANGE => Some("ERROR_INDEX_OUT_OF_RANGE"),
		XrResult::ERROR_VIEW_CONFIGURATION_TYPE_UNSUPPORTED => {
			Some("ERROR_VIEW_CONFIGURATION_TYPE_UNSUPPORTED")
		}
		XrResult::ERROR_ENVIRONMENT_BLEND_MODE_UNSUPPORTED => {
			Some("ERROR_ENVIRONMENT_BLEND_MODE_UNSUPPORTED")
		}
		XrResult::ERROR_NAME_DUPLICATED => Some("ERROR_NAME_DUPLICATED"),
		XrResult::ERROR_NAME_INVALID => Some("ERROR_NAME_INVALID"),
		XrResult::ERROR_ACTIONSET_NOT_ATTACHED => Some("ERROR_ACTIONSET_NOT_ATTACHED"),
		XrResult::ERROR_ACTIONSETS_ALREADY_ATTACHED => Some("ERROR_ACTIONSETS_ALREADY_ATTACHED"),
		XrResult::ERROR_LOCALIZED_NAME_DUPLICATED => Some("ERROR_LOCALIZED_NAME_DUPLICATED"),
		XrResult::ERROR_LOCALIZED_NAME_INVALID => Some("ERROR_LOCALIZED_NAME_INVALID"),
		XrResult::ERROR_GRAPHICS_REQUIREMENTS_CALL_MISSING => {
			Some("ERROR_GRAPHICS_REQUIREMENTS_CALL_MISSING")
		}
		XrResult::ERROR_RUNTIME_UNAVAILABLE => Some("ERROR_RUNTIME_UNAVAILABLE"),
		XrResult::ERROR_ANDROID_THREAD_SETTINGS_ID_INVALID_KHR => {
			Some("ERROR_ANDROID_THREAD_SETTINGS_ID_INVALID_KHR")
		}
		XrResult::ERROR_ANDROID_THREAD_SETTINGS_FAILURE_KHR => {
			Some("ERROR_ANDROID_THREAD_SETTINGS_FAILURE_KHR")
		}
		XrResult::ERROR_CREATE_SPATIAL_ANCHOR_FAILED_MSFT => {
			Some("ERROR_CREATE_SPATIAL_ANCHOR_FAILED_MSFT")
		}
		XrResult::ERROR_SECONDARY_VIEW_CONFIGURATION_TYPE_NOT_ENABLED_MSFT => {
			Some("ERROR_SECONDARY_VIEW_CONFIGURATION_TYPE_NOT_ENABLED_MSFT")
		}
		XrResult::ERROR_CONTROLLER_MODEL_KEY_INVALID_MSFT => {
			Some("ERROR_CONTROLLER_MODEL_KEY_INVALID_MSFT")
		}
		XrResult::ERROR_REPROJECTION_MODE_UNSUPPORTED_MSFT => {
			Some("ERROR_REPROJECTION_MODE_UNSUPPORTED_MSFT")
		}
		XrResult::ERROR_COMPUTE_NEW_SCENE_NOT_COMPLETED_MSFT => {
			Some("ERROR_COMPUTE_NEW_SCENE_NOT_COMPLETED_MSFT")
		}
		XrResult::ERROR_SCENE_COMPONENT_ID_INVALID_MSFT => {
			Some("ERROR_SCENE_COMPONENT_ID_INVALID_MSFT")
		}
		XrResult::ERROR_SCENE_COMPONENT_TYPE_MISMATCH_MSFT => {
			Some("ERROR_SCENE_COMPONENT_TYPE_MISMATCH_MSFT")
		}
		XrResult::ERROR_SCENE_MESH_BUFFER_ID_INVALID_MSFT => {
			Some("ERROR_SCENE_MESH_BUFFER_ID_INVALID_MSFT")
		}
		XrResult::ERROR_SCENE_COMPUTE_FEATURE_INCOMPATIBLE_MSFT => {
			Some("ERROR_SCENE_COMPUTE_FEATURE_INCOMPATIBLE_MSFT")
		}
		XrResult::ERROR_SCENE_COMPUTE_CONSISTENCY_MISMATCH_MSFT => {
			Some("ERROR_SCENE_COMPUTE_CONSISTENCY_MISMATCH_MSFT")
		}
		XrResult::ERROR_DISPLAY_REFRESH_RATE_UNSUPPORTED_FB => {
			Some("ERROR_DISPLAY_REFRESH_RATE_UNSUPPORTED_FB")
		}
		XrResult::ERROR_COLOR_SPACE_UNSUPPORTED_FB => Some("ERROR_COLOR_SPACE_UNSUPPORTED_FB"),
		XrResult::ERROR_SPACE_COMPONENT_NOT_SUPPORTED_FB => {
			Some("ERROR_SPACE_COMPONENT_NOT_SUPPORTED_FB")
		}
		XrResult::ERROR_SPACE_COMPONENT_NOT_ENABLED_FB => {
			Some("ERROR_SPACE_COMPONENT_NOT_ENABLED_FB")
		}
		XrResult::ERROR_SPACE_COMPONENT_STATUS_PENDING_FB => {
			Some("ERROR_SPACE_COMPONENT_STATUS_PENDING_FB")
		}
		XrResult::ERROR_SPACE_COMPONENT_STATUS_ALREADY_SET_FB => {
			Some("ERROR_SPACE_COMPONENT_STATUS_ALREADY_SET_FB")
		}
		XrResult::ERROR_UNEXPECTED_STATE_PASSTHROUGH_FB => {
			Some("ERROR_UNEXPECTED_STATE_PASSTHROUGH_FB")
		}
		XrResult::ERROR_FEATURE_ALREADY_CREATED_PASSTHROUGH_FB => {
			Some("ERROR_FEATURE_ALREADY_CREATED_PASSTHROUGH_FB")
		}
		XrResult::ERROR_FEATURE_REQUIRED_PASSTHROUGH_FB => {
			Some("ERROR_FEATURE_REQUIRED_PASSTHROUGH_FB")
		}
		XrResult::ERROR_NOT_PERMITTED_PASSTHROUGH_FB => Some("ERROR_NOT_PERMITTED_PASSTHROUGH_FB"),
		XrResult::ERROR_INSUFFICIENT_RESOURCES_PASSTHROUGH_FB => {
			Some("ERROR_INSUFFICIENT_RESOURCES_PASSTHROUGH_FB")
		}
		XrResult::ERROR_UNKNOWN_PASSTHROUGH_FB => Some("ERROR_UNKNOWN_PASSTHROUGH_FB"),
		XrResult::ERROR_RENDER_MODEL_KEY_INVALID_FB => Some("ERROR_RENDER_MODEL_KEY_INVALID_FB"),
		XrResult::RENDER_MODEL_UNAVAILABLE_FB => Some("RENDER_MODEL_UNAVAILABLE_FB"),
		XrResult::ERROR_MARKER_NOT_TRACKED_VARJO => Some("ERROR_MARKER_NOT_TRACKED_VARJO"),
		XrResult::ERROR_MARKER_ID_INVALID_VARJO => Some("ERROR_MARKER_ID_INVALID_VARJO"),
		XrResult::ERROR_SPATIAL_ANCHOR_NAME_NOT_FOUND_MSFT => {
			Some("ERROR_SPATIAL_ANCHOR_NAME_NOT_FOUND_MSFT")
		}
		XrResult::ERROR_SPATIAL_ANCHOR_NAME_INVALID_MSFT => {
			Some("ERROR_SPATIAL_ANCHOR_NAME_INVALID_MSFT")
		}
		_ => None,
	}
	.map(|s| format!("XR_{}", s))
	.unwrap_or_else(|| {
		let n = value.into_raw();
		if n.is_positive() {
			format!("XR_UNKNOWN_SUCCESS_{}", n)
		} else {
			format!("XR_UNKNOWN_FAILURE_{}", n)
		}
	});

	(buffer as *mut [c_char; MAX_RESULT_STRING_SIZE])
		.as_mut()
		.unwrap()
		.fill(0);
	std::ptr::copy_nonoverlapping(
		string.as_ptr() as *const i8,
		buffer,
		string.len().min(MAX_RESULT_STRING_SIZE),
	);

	XrResult::SUCCESS
}

/// # Safety
/// https://registry.khronos.org/OpenXR/specs/1.0/html/xrspec.html#xrStructureTypeToString
#[no_mangle]
pub unsafe extern "system" fn xrStructureTypeToString(
	_instance: Instance,
	value: StructureType,
	buffer: *mut c_char,
) -> XrResult {
	let string = match value {
		StructureType::UNKNOWN => Some("UNKNOWN"),
		StructureType::API_LAYER_PROPERTIES => Some("API_LAYER_PROPERTIES"),
		StructureType::EXTENSION_PROPERTIES => Some("EXTENSION_PROPERTIES"),
		StructureType::INSTANCE_CREATE_INFO => Some("INSTANCE_CREATE_INFO"),
		StructureType::SYSTEM_GET_INFO => Some("SYSTEM_GET_INFO"),
		StructureType::SYSTEM_PROPERTIES => Some("SYSTEM_PROPERTIES"),
		StructureType::VIEW_LOCATE_INFO => Some("VIEW_LOCATE_INFO"),
		StructureType::VIEW => Some("VIEW"),
		StructureType::SESSION_CREATE_INFO => Some("SESSION_CREATE_INFO"),
		StructureType::SWAPCHAIN_CREATE_INFO => Some("SWAPCHAIN_CREATE_INFO"),
		StructureType::SESSION_BEGIN_INFO => Some("SESSION_BEGIN_INFO"),
		StructureType::VIEW_STATE => Some("VIEW_STATE"),
		StructureType::FRAME_END_INFO => Some("FRAME_END_INFO"),
		StructureType::HAPTIC_VIBRATION => Some("HAPTIC_VIBRATION"),
		StructureType::EVENT_DATA_BUFFER => Some("EVENT_DATA_BUFFER"),
		StructureType::EVENT_DATA_INSTANCE_LOSS_PENDING => Some("EVENT_DATA_INSTANCE_LOSS_PENDING"),
		StructureType::EVENT_DATA_SESSION_STATE_CHANGED => Some("EVENT_DATA_SESSION_STATE_CHANGED"),
		StructureType::ACTION_STATE_BOOLEAN => Some("ACTION_STATE_BOOLEAN"),
		StructureType::ACTION_STATE_FLOAT => Some("ACTION_STATE_FLOAT"),
		StructureType::ACTION_STATE_VECTOR2F => Some("ACTION_STATE_VECTOR2F"),
		StructureType::ACTION_STATE_POSE => Some("ACTION_STATE_POSE"),
		StructureType::ACTION_SET_CREATE_INFO => Some("ACTION_SET_CREATE_INFO"),
		StructureType::ACTION_CREATE_INFO => Some("ACTION_CREATE_INFO"),
		StructureType::INSTANCE_PROPERTIES => Some("INSTANCE_PROPERTIES"),
		StructureType::FRAME_WAIT_INFO => Some("FRAME_WAIT_INFO"),
		StructureType::COMPOSITION_LAYER_PROJECTION => Some("COMPOSITION_LAYER_PROJECTION"),
		StructureType::COMPOSITION_LAYER_QUAD => Some("COMPOSITION_LAYER_QUAD"),
		StructureType::REFERENCE_SPACE_CREATE_INFO => Some("REFERENCE_SPACE_CREATE_INFO"),
		StructureType::ACTION_SPACE_CREATE_INFO => Some("ACTION_SPACE_CREATE_INFO"),
		StructureType::EVENT_DATA_REFERENCE_SPACE_CHANGE_PENDING => {
			Some("EVENT_DATA_REFERENCE_SPACE_CHANGE_PENDING")
		}
		StructureType::VIEW_CONFIGURATION_VIEW => Some("VIEW_CONFIGURATION_VIEW"),
		StructureType::SPACE_LOCATION => Some("SPACE_LOCATION"),
		StructureType::SPACE_VELOCITY => Some("SPACE_VELOCITY"),
		StructureType::FRAME_STATE => Some("FRAME_STATE"),
		StructureType::VIEW_CONFIGURATION_PROPERTIES => Some("VIEW_CONFIGURATION_PROPERTIES"),
		StructureType::FRAME_BEGIN_INFO => Some("FRAME_BEGIN_INFO"),
		StructureType::COMPOSITION_LAYER_PROJECTION_VIEW => {
			Some("COMPOSITION_LAYER_PROJECTION_VIEW")
		}
		StructureType::EVENT_DATA_EVENTS_LOST => Some("EVENT_DATA_EVENTS_LOST"),
		StructureType::INTERACTION_PROFILE_SUGGESTED_BINDING => {
			Some("INTERACTION_PROFILE_SUGGESTED_BINDING")
		}
		StructureType::EVENT_DATA_INTERACTION_PROFILE_CHANGED => {
			Some("EVENT_DATA_INTERACTION_PROFILE_CHANGED")
		}
		StructureType::INTERACTION_PROFILE_STATE => Some("INTERACTION_PROFILE_STATE"),
		StructureType::SWAPCHAIN_IMAGE_ACQUIRE_INFO => Some("SWAPCHAIN_IMAGE_ACQUIRE_INFO"),
		StructureType::SWAPCHAIN_IMAGE_WAIT_INFO => Some("SWAPCHAIN_IMAGE_WAIT_INFO"),
		StructureType::SWAPCHAIN_IMAGE_RELEASE_INFO => Some("SWAPCHAIN_IMAGE_RELEASE_INFO"),
		StructureType::ACTION_STATE_GET_INFO => Some("ACTION_STATE_GET_INFO"),
		StructureType::HAPTIC_ACTION_INFO => Some("HAPTIC_ACTION_INFO"),
		StructureType::SESSION_ACTION_SETS_ATTACH_INFO => Some("SESSION_ACTION_SETS_ATTACH_INFO"),
		StructureType::ACTIONS_SYNC_INFO => Some("ACTIONS_SYNC_INFO"),
		StructureType::BOUND_SOURCES_FOR_ACTION_ENUMERATE_INFO => {
			Some("BOUND_SOURCES_FOR_ACTION_ENUMERATE_INFO")
		}
		StructureType::INPUT_SOURCE_LOCALIZED_NAME_GET_INFO => {
			Some("INPUT_SOURCE_LOCALIZED_NAME_GET_INFO")
		}
		StructureType::COMPOSITION_LAYER_CUBE_KHR => Some("COMPOSITION_LAYER_CUBE_KHR"),
		StructureType::INSTANCE_CREATE_INFO_ANDROID_KHR => Some("INSTANCE_CREATE_INFO_ANDROID_KHR"),
		StructureType::COMPOSITION_LAYER_DEPTH_INFO_KHR => Some("COMPOSITION_LAYER_DEPTH_INFO_KHR"),
		StructureType::VULKAN_SWAPCHAIN_FORMAT_LIST_CREATE_INFO_KHR => {
			Some("VULKAN_SWAPCHAIN_FORMAT_LIST_CREATE_INFO_KHR")
		}
		StructureType::EVENT_DATA_PERF_SETTINGS_EXT => Some("EVENT_DATA_PERF_SETTINGS_EXT"),
		StructureType::COMPOSITION_LAYER_CYLINDER_KHR => Some("COMPOSITION_LAYER_CYLINDER_KHR"),
		StructureType::COMPOSITION_LAYER_EQUIRECT_KHR => Some("COMPOSITION_LAYER_EQUIRECT_KHR"),
		StructureType::DEBUG_UTILS_OBJECT_NAME_INFO_EXT => Some("DEBUG_UTILS_OBJECT_NAME_INFO_EXT"),
		StructureType::DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT => {
			Some("DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT")
		}
		StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT => {
			Some("DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT")
		}
		StructureType::DEBUG_UTILS_LABEL_EXT => Some("DEBUG_UTILS_LABEL_EXT"),
		StructureType::GRAPHICS_BINDING_OPENGL_WIN32_KHR => {
			Some("GRAPHICS_BINDING_OPENGL_WIN32_KHR")
		}
		StructureType::GRAPHICS_BINDING_OPENGL_XLIB_KHR => Some("GRAPHICS_BINDING_OPENGL_XLIB_KHR"),
		StructureType::GRAPHICS_BINDING_OPENGL_XCB_KHR => Some("GRAPHICS_BINDING_OPENGL_XCB_KHR"),
		StructureType::GRAPHICS_BINDING_OPENGL_WAYLAND_KHR => {
			Some("GRAPHICS_BINDING_OPENGL_WAYLAND_KHR")
		}
		StructureType::SWAPCHAIN_IMAGE_OPENGL_KHR => Some("SWAPCHAIN_IMAGE_OPENGL_KHR"),
		StructureType::GRAPHICS_REQUIREMENTS_OPENGL_KHR => Some("GRAPHICS_REQUIREMENTS_OPENGL_KHR"),
		StructureType::GRAPHICS_BINDING_OPENGL_ES_ANDROID_KHR => {
			Some("GRAPHICS_BINDING_OPENGL_ES_ANDROID_KHR")
		}
		StructureType::SWAPCHAIN_IMAGE_OPENGL_ES_KHR => Some("SWAPCHAIN_IMAGE_OPENGL_ES_KHR"),
		StructureType::GRAPHICS_REQUIREMENTS_OPENGL_ES_KHR => {
			Some("GRAPHICS_REQUIREMENTS_OPENGL_ES_KHR")
		}
		StructureType::GRAPHICS_BINDING_VULKAN_KHR => Some("GRAPHICS_BINDING_VULKAN_KHR"),
		StructureType::SWAPCHAIN_IMAGE_VULKAN_KHR => Some("SWAPCHAIN_IMAGE_VULKAN_KHR"),
		StructureType::GRAPHICS_REQUIREMENTS_VULKAN_KHR => Some("GRAPHICS_REQUIREMENTS_VULKAN_KHR"),
		StructureType::GRAPHICS_BINDING_D3D11_KHR => Some("GRAPHICS_BINDING_D3D11_KHR"),
		StructureType::SWAPCHAIN_IMAGE_D3D11_KHR => Some("SWAPCHAIN_IMAGE_D3D11_KHR"),
		StructureType::GRAPHICS_REQUIREMENTS_D3D11_KHR => Some("GRAPHICS_REQUIREMENTS_D3D11_KHR"),
		StructureType::GRAPHICS_BINDING_D3D12_KHR => Some("GRAPHICS_BINDING_D3D12_KHR"),
		StructureType::SWAPCHAIN_IMAGE_D3D12_KHR => Some("SWAPCHAIN_IMAGE_D3D12_KHR"),
		StructureType::GRAPHICS_REQUIREMENTS_D3D12_KHR => Some("GRAPHICS_REQUIREMENTS_D3D12_KHR"),
		StructureType::SYSTEM_EYE_GAZE_INTERACTION_PROPERTIES_EXT => {
			Some("SYSTEM_EYE_GAZE_INTERACTION_PROPERTIES_EXT")
		}
		StructureType::EYE_GAZE_SAMPLE_TIME_EXT => Some("EYE_GAZE_SAMPLE_TIME_EXT"),
		StructureType::VISIBILITY_MASK_KHR => Some("VISIBILITY_MASK_KHR"),
		StructureType::EVENT_DATA_VISIBILITY_MASK_CHANGED_KHR => {
			Some("EVENT_DATA_VISIBILITY_MASK_CHANGED_KHR")
		}
		StructureType::SESSION_CREATE_INFO_OVERLAY_EXTX => Some("SESSION_CREATE_INFO_OVERLAY_EXTX"),
		StructureType::EVENT_DATA_MAIN_SESSION_VISIBILITY_CHANGED_EXTX => {
			Some("EVENT_DATA_MAIN_SESSION_VISIBILITY_CHANGED_EXTX")
		}
		StructureType::COMPOSITION_LAYER_COLOR_SCALE_BIAS_KHR => {
			Some("COMPOSITION_LAYER_COLOR_SCALE_BIAS_KHR")
		}
		StructureType::SPATIAL_ANCHOR_CREATE_INFO_MSFT => Some("SPATIAL_ANCHOR_CREATE_INFO_MSFT"),
		StructureType::SPATIAL_ANCHOR_SPACE_CREATE_INFO_MSFT => {
			Some("SPATIAL_ANCHOR_SPACE_CREATE_INFO_MSFT")
		}
		StructureType::COMPOSITION_LAYER_IMAGE_LAYOUT_FB => {
			Some("COMPOSITION_LAYER_IMAGE_LAYOUT_FB")
		}
		StructureType::COMPOSITION_LAYER_ALPHA_BLEND_FB => Some("COMPOSITION_LAYER_ALPHA_BLEND_FB"),
		StructureType::VIEW_CONFIGURATION_DEPTH_RANGE_EXT => {
			Some("VIEW_CONFIGURATION_DEPTH_RANGE_EXT")
		}
		StructureType::GRAPHICS_BINDING_EGL_MNDX => Some("GRAPHICS_BINDING_EGL_MNDX"),
		StructureType::SPATIAL_GRAPH_NODE_SPACE_CREATE_INFO_MSFT => {
			Some("SPATIAL_GRAPH_NODE_SPACE_CREATE_INFO_MSFT")
		}
		StructureType::SPATIAL_GRAPH_STATIC_NODE_BINDING_CREATE_INFO_MSFT => {
			Some("SPATIAL_GRAPH_STATIC_NODE_BINDING_CREATE_INFO_MSFT")
		}
		StructureType::SPATIAL_GRAPH_NODE_BINDING_PROPERTIES_GET_INFO_MSFT => {
			Some("SPATIAL_GRAPH_NODE_BINDING_PROPERTIES_GET_INFO_MSFT")
		}
		StructureType::SPATIAL_GRAPH_NODE_BINDING_PROPERTIES_MSFT => {
			Some("SPATIAL_GRAPH_NODE_BINDING_PROPERTIES_MSFT")
		}
		StructureType::SYSTEM_HAND_TRACKING_PROPERTIES_EXT => {
			Some("SYSTEM_HAND_TRACKING_PROPERTIES_EXT")
		}
		StructureType::HAND_TRACKER_CREATE_INFO_EXT => Some("HAND_TRACKER_CREATE_INFO_EXT"),
		StructureType::HAND_JOINTS_LOCATE_INFO_EXT => Some("HAND_JOINTS_LOCATE_INFO_EXT"),
		StructureType::HAND_JOINT_LOCATIONS_EXT => Some("HAND_JOINT_LOCATIONS_EXT"),
		StructureType::HAND_JOINT_VELOCITIES_EXT => Some("HAND_JOINT_VELOCITIES_EXT"),
		StructureType::SYSTEM_HAND_TRACKING_MESH_PROPERTIES_MSFT => {
			Some("SYSTEM_HAND_TRACKING_MESH_PROPERTIES_MSFT")
		}
		StructureType::HAND_MESH_SPACE_CREATE_INFO_MSFT => Some("HAND_MESH_SPACE_CREATE_INFO_MSFT"),
		StructureType::HAND_MESH_UPDATE_INFO_MSFT => Some("HAND_MESH_UPDATE_INFO_MSFT"),
		StructureType::HAND_MESH_MSFT => Some("HAND_MESH_MSFT"),
		StructureType::HAND_POSE_TYPE_INFO_MSFT => Some("HAND_POSE_TYPE_INFO_MSFT"),
		StructureType::SECONDARY_VIEW_CONFIGURATION_SESSION_BEGIN_INFO_MSFT => {
			Some("SECONDARY_VIEW_CONFIGURATION_SESSION_BEGIN_INFO_MSFT")
		}
		StructureType::SECONDARY_VIEW_CONFIGURATION_STATE_MSFT => {
			Some("SECONDARY_VIEW_CONFIGURATION_STATE_MSFT")
		}
		StructureType::SECONDARY_VIEW_CONFIGURATION_FRAME_STATE_MSFT => {
			Some("SECONDARY_VIEW_CONFIGURATION_FRAME_STATE_MSFT")
		}
		StructureType::SECONDARY_VIEW_CONFIGURATION_FRAME_END_INFO_MSFT => {
			Some("SECONDARY_VIEW_CONFIGURATION_FRAME_END_INFO_MSFT")
		}
		StructureType::SECONDARY_VIEW_CONFIGURATION_LAYER_INFO_MSFT => {
			Some("SECONDARY_VIEW_CONFIGURATION_LAYER_INFO_MSFT")
		}
		StructureType::SECONDARY_VIEW_CONFIGURATION_SWAPCHAIN_CREATE_INFO_MSFT => {
			Some("SECONDARY_VIEW_CONFIGURATION_SWAPCHAIN_CREATE_INFO_MSFT")
		}
		StructureType::CONTROLLER_MODEL_KEY_STATE_MSFT => Some("CONTROLLER_MODEL_KEY_STATE_MSFT"),
		StructureType::CONTROLLER_MODEL_NODE_PROPERTIES_MSFT => {
			Some("CONTROLLER_MODEL_NODE_PROPERTIES_MSFT")
		}
		StructureType::CONTROLLER_MODEL_PROPERTIES_MSFT => Some("CONTROLLER_MODEL_PROPERTIES_MSFT"),
		StructureType::CONTROLLER_MODEL_NODE_STATE_MSFT => Some("CONTROLLER_MODEL_NODE_STATE_MSFT"),
		StructureType::CONTROLLER_MODEL_STATE_MSFT => Some("CONTROLLER_MODEL_STATE_MSFT"),
		StructureType::VIEW_CONFIGURATION_VIEW_FOV_EPIC => Some("VIEW_CONFIGURATION_VIEW_FOV_EPIC"),
		StructureType::HOLOGRAPHIC_WINDOW_ATTACHMENT_MSFT => {
			Some("HOLOGRAPHIC_WINDOW_ATTACHMENT_MSFT")
		}
		StructureType::COMPOSITION_LAYER_REPROJECTION_INFO_MSFT => {
			Some("COMPOSITION_LAYER_REPROJECTION_INFO_MSFT")
		}
		StructureType::COMPOSITION_LAYER_REPROJECTION_PLANE_OVERRIDE_MSFT => {
			Some("COMPOSITION_LAYER_REPROJECTION_PLANE_OVERRIDE_MSFT")
		}
		StructureType::ANDROID_SURFACE_SWAPCHAIN_CREATE_INFO_FB => {
			Some("ANDROID_SURFACE_SWAPCHAIN_CREATE_INFO_FB")
		}
		StructureType::COMPOSITION_LAYER_SECURE_CONTENT_FB => {
			Some("COMPOSITION_LAYER_SECURE_CONTENT_FB")
		}
		StructureType::INTERACTION_PROFILE_DPAD_BINDING_EXT => {
			Some("INTERACTION_PROFILE_DPAD_BINDING_EXT")
		}
		StructureType::INTERACTION_PROFILE_ANALOG_THRESHOLD_VALVE => {
			Some("INTERACTION_PROFILE_ANALOG_THRESHOLD_VALVE")
		}
		StructureType::HAND_JOINTS_MOTION_RANGE_INFO_EXT => {
			Some("HAND_JOINTS_MOTION_RANGE_INFO_EXT")
		}
		StructureType::LOADER_INIT_INFO_ANDROID_KHR => Some("LOADER_INIT_INFO_ANDROID_KHR"),
		StructureType::VULKAN_INSTANCE_CREATE_INFO_KHR => Some("VULKAN_INSTANCE_CREATE_INFO_KHR"),
		StructureType::VULKAN_DEVICE_CREATE_INFO_KHR => Some("VULKAN_DEVICE_CREATE_INFO_KHR"),
		StructureType::VULKAN_GRAPHICS_DEVICE_GET_INFO_KHR => {
			Some("VULKAN_GRAPHICS_DEVICE_GET_INFO_KHR")
		}
		StructureType::COMPOSITION_LAYER_EQUIRECT2_KHR => Some("COMPOSITION_LAYER_EQUIRECT2_KHR"),
		StructureType::SCENE_OBSERVER_CREATE_INFO_MSFT => Some("SCENE_OBSERVER_CREATE_INFO_MSFT"),
		StructureType::SCENE_CREATE_INFO_MSFT => Some("SCENE_CREATE_INFO_MSFT"),
		StructureType::NEW_SCENE_COMPUTE_INFO_MSFT => Some("NEW_SCENE_COMPUTE_INFO_MSFT"),
		StructureType::VISUAL_MESH_COMPUTE_LOD_INFO_MSFT => {
			Some("VISUAL_MESH_COMPUTE_LOD_INFO_MSFT")
		}
		StructureType::SCENE_COMPONENTS_MSFT => Some("SCENE_COMPONENTS_MSFT"),
		StructureType::SCENE_COMPONENTS_GET_INFO_MSFT => Some("SCENE_COMPONENTS_GET_INFO_MSFT"),
		StructureType::SCENE_COMPONENT_LOCATIONS_MSFT => Some("SCENE_COMPONENT_LOCATIONS_MSFT"),
		StructureType::SCENE_COMPONENTS_LOCATE_INFO_MSFT => {
			Some("SCENE_COMPONENTS_LOCATE_INFO_MSFT")
		}
		StructureType::SCENE_OBJECTS_MSFT => Some("SCENE_OBJECTS_MSFT"),
		StructureType::SCENE_COMPONENT_PARENT_FILTER_INFO_MSFT => {
			Some("SCENE_COMPONENT_PARENT_FILTER_INFO_MSFT")
		}
		StructureType::SCENE_OBJECT_TYPES_FILTER_INFO_MSFT => {
			Some("SCENE_OBJECT_TYPES_FILTER_INFO_MSFT")
		}
		StructureType::SCENE_PLANES_MSFT => Some("SCENE_PLANES_MSFT"),
		StructureType::SCENE_PLANE_ALIGNMENT_FILTER_INFO_MSFT => {
			Some("SCENE_PLANE_ALIGNMENT_FILTER_INFO_MSFT")
		}
		StructureType::SCENE_MESHES_MSFT => Some("SCENE_MESHES_MSFT"),
		StructureType::SCENE_MESH_BUFFERS_GET_INFO_MSFT => Some("SCENE_MESH_BUFFERS_GET_INFO_MSFT"),
		StructureType::SCENE_MESH_BUFFERS_MSFT => Some("SCENE_MESH_BUFFERS_MSFT"),
		StructureType::SCENE_MESH_VERTEX_BUFFER_MSFT => Some("SCENE_MESH_VERTEX_BUFFER_MSFT"),
		StructureType::SCENE_MESH_INDICES_UINT32_MSFT => Some("SCENE_MESH_INDICES_UINT32_MSFT"),
		StructureType::SCENE_MESH_INDICES_UINT16_MSFT => Some("SCENE_MESH_INDICES_UINT16_MSFT"),
		StructureType::SERIALIZED_SCENE_FRAGMENT_DATA_GET_INFO_MSFT => {
			Some("SERIALIZED_SCENE_FRAGMENT_DATA_GET_INFO_MSFT")
		}
		StructureType::SCENE_DESERIALIZE_INFO_MSFT => Some("SCENE_DESERIALIZE_INFO_MSFT"),
		StructureType::EVENT_DATA_DISPLAY_REFRESH_RATE_CHANGED_FB => {
			Some("EVENT_DATA_DISPLAY_REFRESH_RATE_CHANGED_FB")
		}
		StructureType::VIVE_TRACKER_PATHS_HTCX => Some("VIVE_TRACKER_PATHS_HTCX"),
		StructureType::EVENT_DATA_VIVE_TRACKER_CONNECTED_HTCX => {
			Some("EVENT_DATA_VIVE_TRACKER_CONNECTED_HTCX")
		}
		StructureType::SYSTEM_FACIAL_TRACKING_PROPERTIES_HTC => {
			Some("SYSTEM_FACIAL_TRACKING_PROPERTIES_HTC")
		}
		StructureType::FACIAL_TRACKER_CREATE_INFO_HTC => Some("FACIAL_TRACKER_CREATE_INFO_HTC"),
		StructureType::FACIAL_EXPRESSIONS_HTC => Some("FACIAL_EXPRESSIONS_HTC"),
		StructureType::SYSTEM_COLOR_SPACE_PROPERTIES_FB => Some("SYSTEM_COLOR_SPACE_PROPERTIES_FB"),
		StructureType::HAND_TRACKING_MESH_FB => Some("HAND_TRACKING_MESH_FB"),
		StructureType::HAND_TRACKING_SCALE_FB => Some("HAND_TRACKING_SCALE_FB"),
		StructureType::HAND_TRACKING_AIM_STATE_FB => Some("HAND_TRACKING_AIM_STATE_FB"),
		StructureType::HAND_TRACKING_CAPSULES_STATE_FB => Some("HAND_TRACKING_CAPSULES_STATE_FB"),
		StructureType::SYSTEM_SPATIAL_ENTITY_PROPERTIES_FB => {
			Some("SYSTEM_SPATIAL_ENTITY_PROPERTIES_FB")
		}
		StructureType::SPATIAL_ANCHOR_CREATE_INFO_FB => Some("SPATIAL_ANCHOR_CREATE_INFO_FB"),
		StructureType::SPACE_COMPONENT_STATUS_SET_INFO_FB => {
			Some("SPACE_COMPONENT_STATUS_SET_INFO_FB")
		}
		StructureType::SPACE_COMPONENT_STATUS_FB => Some("SPACE_COMPONENT_STATUS_FB"),
		StructureType::EVENT_DATA_SPATIAL_ANCHOR_CREATE_COMPLETE_FB => {
			Some("EVENT_DATA_SPATIAL_ANCHOR_CREATE_COMPLETE_FB")
		}
		StructureType::EVENT_DATA_SPACE_SET_STATUS_COMPLETE_FB => {
			Some("EVENT_DATA_SPACE_SET_STATUS_COMPLETE_FB")
		}
		StructureType::FOVEATION_PROFILE_CREATE_INFO_FB => Some("FOVEATION_PROFILE_CREATE_INFO_FB"),
		StructureType::SWAPCHAIN_CREATE_INFO_FOVEATION_FB => {
			Some("SWAPCHAIN_CREATE_INFO_FOVEATION_FB")
		}
		StructureType::SWAPCHAIN_STATE_FOVEATION_FB => Some("SWAPCHAIN_STATE_FOVEATION_FB"),
		StructureType::FOVEATION_LEVEL_PROFILE_CREATE_INFO_FB => {
			Some("FOVEATION_LEVEL_PROFILE_CREATE_INFO_FB")
		}
		StructureType::KEYBOARD_SPACE_CREATE_INFO_FB => Some("KEYBOARD_SPACE_CREATE_INFO_FB"),
		StructureType::KEYBOARD_TRACKING_QUERY_FB => Some("KEYBOARD_TRACKING_QUERY_FB"),
		StructureType::SYSTEM_KEYBOARD_TRACKING_PROPERTIES_FB => {
			Some("SYSTEM_KEYBOARD_TRACKING_PROPERTIES_FB")
		}
		StructureType::TRIANGLE_MESH_CREATE_INFO_FB => Some("TRIANGLE_MESH_CREATE_INFO_FB"),
		StructureType::SYSTEM_PASSTHROUGH_PROPERTIES_FB => Some("SYSTEM_PASSTHROUGH_PROPERTIES_FB"),
		StructureType::PASSTHROUGH_CREATE_INFO_FB => Some("PASSTHROUGH_CREATE_INFO_FB"),
		StructureType::PASSTHROUGH_LAYER_CREATE_INFO_FB => Some("PASSTHROUGH_LAYER_CREATE_INFO_FB"),
		StructureType::COMPOSITION_LAYER_PASSTHROUGH_FB => Some("COMPOSITION_LAYER_PASSTHROUGH_FB"),
		StructureType::GEOMETRY_INSTANCE_CREATE_INFO_FB => Some("GEOMETRY_INSTANCE_CREATE_INFO_FB"),
		StructureType::GEOMETRY_INSTANCE_TRANSFORM_FB => Some("GEOMETRY_INSTANCE_TRANSFORM_FB"),
		StructureType::SYSTEM_PASSTHROUGH_PROPERTIES2_FB => {
			Some("SYSTEM_PASSTHROUGH_PROPERTIES2_FB")
		}
		StructureType::PASSTHROUGH_STYLE_FB => Some("PASSTHROUGH_STYLE_FB"),
		StructureType::PASSTHROUGH_COLOR_MAP_MONO_TO_RGBA_FB => {
			Some("PASSTHROUGH_COLOR_MAP_MONO_TO_RGBA_FB")
		}
		StructureType::PASSTHROUGH_COLOR_MAP_MONO_TO_MONO_FB => {
			Some("PASSTHROUGH_COLOR_MAP_MONO_TO_MONO_FB")
		}
		StructureType::PASSTHROUGH_BRIGHTNESS_CONTRAST_SATURATION_FB => {
			Some("PASSTHROUGH_BRIGHTNESS_CONTRAST_SATURATION_FB")
		}
		StructureType::EVENT_DATA_PASSTHROUGH_STATE_CHANGED_FB => {
			Some("EVENT_DATA_PASSTHROUGH_STATE_CHANGED_FB")
		}
		StructureType::RENDER_MODEL_PATH_INFO_FB => Some("RENDER_MODEL_PATH_INFO_FB"),
		StructureType::RENDER_MODEL_PROPERTIES_FB => Some("RENDER_MODEL_PROPERTIES_FB"),
		StructureType::RENDER_MODEL_BUFFER_FB => Some("RENDER_MODEL_BUFFER_FB"),
		StructureType::RENDER_MODEL_LOAD_INFO_FB => Some("RENDER_MODEL_LOAD_INFO_FB"),
		StructureType::SYSTEM_RENDER_MODEL_PROPERTIES_FB => {
			Some("SYSTEM_RENDER_MODEL_PROPERTIES_FB")
		}
		StructureType::RENDER_MODEL_CAPABILITIES_REQUEST_FB => {
			Some("RENDER_MODEL_CAPABILITIES_REQUEST_FB")
		}
		StructureType::BINDING_MODIFICATIONS_KHR => Some("BINDING_MODIFICATIONS_KHR"),
		StructureType::VIEW_LOCATE_FOVEATED_RENDERING_VARJO => {
			Some("VIEW_LOCATE_FOVEATED_RENDERING_VARJO")
		}
		StructureType::FOVEATED_VIEW_CONFIGURATION_VIEW_VARJO => {
			Some("FOVEATED_VIEW_CONFIGURATION_VIEW_VARJO")
		}
		StructureType::SYSTEM_FOVEATED_RENDERING_PROPERTIES_VARJO => {
			Some("SYSTEM_FOVEATED_RENDERING_PROPERTIES_VARJO")
		}
		StructureType::COMPOSITION_LAYER_DEPTH_TEST_VARJO => {
			Some("COMPOSITION_LAYER_DEPTH_TEST_VARJO")
		}
		StructureType::SYSTEM_MARKER_TRACKING_PROPERTIES_VARJO => {
			Some("SYSTEM_MARKER_TRACKING_PROPERTIES_VARJO")
		}
		StructureType::EVENT_DATA_MARKER_TRACKING_UPDATE_VARJO => {
			Some("EVENT_DATA_MARKER_TRACKING_UPDATE_VARJO")
		}
		StructureType::MARKER_SPACE_CREATE_INFO_VARJO => Some("MARKER_SPACE_CREATE_INFO_VARJO"),
		StructureType::SPATIAL_ANCHOR_PERSISTENCE_INFO_MSFT => {
			Some("SPATIAL_ANCHOR_PERSISTENCE_INFO_MSFT")
		}
		StructureType::SPATIAL_ANCHOR_FROM_PERSISTED_ANCHOR_CREATE_INFO_MSFT => {
			Some("SPATIAL_ANCHOR_FROM_PERSISTED_ANCHOR_CREATE_INFO_MSFT")
		}
		StructureType::SPACE_QUERY_INFO_FB => Some("SPACE_QUERY_INFO_FB"),
		StructureType::SPACE_QUERY_RESULTS_FB => Some("SPACE_QUERY_RESULTS_FB"),
		StructureType::SPACE_STORAGE_LOCATION_FILTER_INFO_FB => {
			Some("SPACE_STORAGE_LOCATION_FILTER_INFO_FB")
		}
		StructureType::SPACE_UUID_FILTER_INFO_FB => Some("SPACE_UUID_FILTER_INFO_FB"),
		StructureType::SPACE_COMPONENT_FILTER_INFO_FB => Some("SPACE_COMPONENT_FILTER_INFO_FB"),
		StructureType::EVENT_DATA_SPACE_QUERY_RESULTS_AVAILABLE_FB => {
			Some("EVENT_DATA_SPACE_QUERY_RESULTS_AVAILABLE_FB")
		}
		StructureType::EVENT_DATA_SPACE_QUERY_COMPLETE_FB => {
			Some("EVENT_DATA_SPACE_QUERY_COMPLETE_FB")
		}
		StructureType::SPACE_SAVE_INFO_FB => Some("SPACE_SAVE_INFO_FB"),
		StructureType::SPACE_ERASE_INFO_FB => Some("SPACE_ERASE_INFO_FB"),
		StructureType::EVENT_DATA_SPACE_SAVE_COMPLETE_FB => {
			Some("EVENT_DATA_SPACE_SAVE_COMPLETE_FB")
		}
		StructureType::EVENT_DATA_SPACE_ERASE_COMPLETE_FB => {
			Some("EVENT_DATA_SPACE_ERASE_COMPLETE_FB")
		}
		StructureType::SWAPCHAIN_IMAGE_FOVEATION_VULKAN_FB => {
			Some("SWAPCHAIN_IMAGE_FOVEATION_VULKAN_FB")
		}
		StructureType::SWAPCHAIN_STATE_ANDROID_SURFACE_DIMENSIONS_FB => {
			Some("SWAPCHAIN_STATE_ANDROID_SURFACE_DIMENSIONS_FB")
		}
		StructureType::SWAPCHAIN_STATE_SAMPLER_OPENGL_ES_FB => {
			Some("SWAPCHAIN_STATE_SAMPLER_OPENGL_ES_FB")
		}
		StructureType::SWAPCHAIN_STATE_SAMPLER_VULKAN_FB => {
			Some("SWAPCHAIN_STATE_SAMPLER_VULKAN_FB")
		}
		StructureType::COMPOSITION_LAYER_SPACE_WARP_INFO_FB => {
			Some("COMPOSITION_LAYER_SPACE_WARP_INFO_FB")
		}
		StructureType::SYSTEM_SPACE_WARP_PROPERTIES_FB => Some("SYSTEM_SPACE_WARP_PROPERTIES_FB"),
		StructureType::SEMANTIC_LABELS_FB => Some("SEMANTIC_LABELS_FB"),
		StructureType::ROOM_LAYOUT_FB => Some("ROOM_LAYOUT_FB"),
		StructureType::BOUNDARY_2D_FB => Some("BOUNDARY_2D_FB"),
		StructureType::DIGITAL_LENS_CONTROL_ALMALENCE => Some("DIGITAL_LENS_CONTROL_ALMALENCE"),
		StructureType::SPACE_CONTAINER_FB => Some("SPACE_CONTAINER_FB"),
		StructureType::PASSTHROUGH_KEYBOARD_HANDS_INTENSITY_FB => {
			Some("PASSTHROUGH_KEYBOARD_HANDS_INTENSITY_FB")
		}
		StructureType::COMPOSITION_LAYER_SETTINGS_FB => Some("COMPOSITION_LAYER_SETTINGS_FB"),
		StructureType::VULKAN_SWAPCHAIN_CREATE_INFO_META => {
			Some("VULKAN_SWAPCHAIN_CREATE_INFO_META")
		}
		StructureType::PERFORMANCE_METRICS_STATE_META => Some("PERFORMANCE_METRICS_STATE_META"),
		StructureType::PERFORMANCE_METRICS_COUNTER_META => Some("PERFORMANCE_METRICS_COUNTER_META"),
		_ => None,
	}
	.map(|s| format!("XR_{}", s))
	.unwrap_or_else(|| format!("XR_UNKNOWN_STRUCTURE_TYPE_{}", value.into_raw()));

	(buffer as *mut [c_char; MAX_STRUCTURE_NAME_SIZE])
		.as_mut()
		.unwrap()
		.fill(0);
	std::ptr::copy_nonoverlapping(
		string.as_ptr() as *const i8,
		buffer,
		string.len().min(MAX_STRUCTURE_NAME_SIZE),
	);

	XrResult::SUCCESS
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrStringToPath
#[no_mangle]
pub unsafe extern "system" fn xrStringToPath(
	instance: Instance,
	path_string: *const c_char,
	path: &mut Path,
) -> XrResult {
	wrap_oxr! {
		let stardust_instance = instance.get_stardust()?;
		let path_string = str_from_const_char(path_string)?;
		let key = stardust_instance.paths.insert(path_string.to_string());
		*path = Path::from_raw(key.data().as_ffi());
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrPathToString
#[no_mangle]
pub unsafe extern "system" fn xrPathToString(
	instance: Instance,
	path: Path,
	buffer_capacity_input: u32,
	buffer_count_output: &mut Option<u32>,
	buffer: *mut c_char,
) -> XrResult {
	wrap_oxr! {
		let path = instance.get_stardust()?.path(path)?.into_bytes().into_iter().map(|c| c as i8).collect::<Vec<_>>();
		enumerate(buffer_capacity_input, buffer_count_output, buffer, &path)?;
	}
}
