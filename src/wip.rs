use crate::XrResult;
use openxr_sys::*;
use std::ffi::c_char;

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroySpace
#[no_mangle]
pub unsafe extern "system" fn xrDestroySpace(_space: Space) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateSwapchainFormats
#[no_mangle]
pub unsafe extern "system" fn xrEnumerateSwapchainFormats(
	_session: Session,
	_format_capacity_input: u32,
	_format_count_output: &mut u32,
	_formats: &mut i64,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateSwapchain
#[no_mangle]
pub unsafe extern "system" fn xrCreateSwapchain(
	_session: Session,
	_create_info: &SwapchainCreateInfo,
	_swapchain: &mut Swapchain,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroySwapchain
#[no_mangle]
pub unsafe extern "system" fn xrDestroySwapchain(_swapchain: Swapchain) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateSwapchainImages
#[no_mangle]
pub unsafe extern "system" fn xrEnumerateSwapchainImages(
	_swapchain: Swapchain,
	_image_capacity_input: u32,
	_image_count_output: &mut u32,
	_images: &mut SwapchainImageBaseHeader,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrAcquireSwapchainImage
#[no_mangle]
pub unsafe extern "system" fn xrAcquireSwapchainImage(
	_swapchain: Swapchain,
	_acquire_info: &SwapchainImageAcquireInfo,
	_index: &mut u32,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrWaitSwapchainImage
#[no_mangle]
pub unsafe extern "system" fn xrWaitSwapchainImage(
	_swapchain: Swapchain,
	_wait_info: &SwapchainImageWaitInfo,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrReleaseSwapchainImage
#[no_mangle]
pub unsafe extern "system" fn xrReleaseSwapchainImage(
	_swapchain: Swapchain,
	_release_info: &SwapchainImageReleaseInfo,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrBeginSession
#[no_mangle]
pub unsafe extern "system" fn xrBeginSession(
	_session: Session,
	_begin_info: &SessionBeginInfo,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEndSession
#[no_mangle]
pub unsafe extern "system" fn xrEndSession(_session: Session) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrRequestExitSession
#[no_mangle]
pub unsafe extern "system" fn xrRequestExitSession(_session: Session) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateReferenceSpaces
#[no_mangle]
pub unsafe extern "system" fn xrEnumerateReferenceSpaces(
	_session: Session,
	_space_capacity_input: u32,
	_space_count_output: &mut u32,
	_spaces: &mut ReferenceSpaceType,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateReferenceSpace
#[no_mangle]
pub unsafe extern "system" fn xrCreateReferenceSpace(
	_session: Session,
	_create_info: &ReferenceSpaceCreateInfo,
	_space: &mut Space,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateActionSpace
#[no_mangle]
pub unsafe extern "system" fn xrCreateActionSpace(
	_session: Session,
	_create_info: &ActionSpaceCreateInfo,
	_space: &mut Space,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrLocateSpace
#[no_mangle]
pub unsafe extern "system" fn xrLocateSpace(
	_space: Space,
	_base_space: Space,
	_time: Time,
	_location: &mut SpaceLocation,
) -> XrResult {
	wrap_oxr! {
		todo!();
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

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrBeginFrame
#[no_mangle]
pub unsafe extern "system" fn xrBeginFrame(
	_session: Session,
	_frame_begin_info: &FrameBeginInfo,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrLocateViews
#[no_mangle]
pub unsafe extern "system" fn xrLocateViews(
	_session: Session,
	_view_locate_info: &ViewLocateInfo,
	_view_state: &mut ViewState,
	_view_capacity_input: u32,
	_view_count_output: &mut u32,
	_views: &mut View,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEndFrame
#[no_mangle]
pub unsafe extern "system" fn xrEndFrame(
	_session: Session,
	_frame_end_info: &FrameEndInfo,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrWaitFrame
#[no_mangle]
pub unsafe extern "system" fn xrWaitFrame(
	_session: Session,
	_frame_wait_info: &FrameWaitInfo,
	_frame_state: &mut FrameState,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrApplyHapticFeedback
#[no_mangle]
pub unsafe extern "system" fn xrApplyHapticFeedback(
	_session: Session,
	_haptic_action_info: &HapticActionInfo,
	_haptic_feedback: &HapticBaseHeader,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrStopHapticFeedback
#[no_mangle]
pub unsafe extern "system" fn xrStopHapticFeedback(
	_session: Session,
	_haptic_action_info: &HapticActionInfo,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrPollEvent
#[no_mangle]
pub unsafe extern "system" fn xrPollEvent(
	_instance: Instance,
	_event_data: &mut EventDataBuffer,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrStringToPath
#[no_mangle]
pub unsafe extern "system" fn xrStringToPath(
	_instance: Instance,
	_path_string: &c_char,
	_path: &mut Path,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrPathToString
#[no_mangle]
pub unsafe extern "system" fn xrPathToString(
	_instance: Instance,
	_path: Path,
	_buffer_capacity_input: u32,
	_buffer_count_output: &mut u32,
	_buffer: &mut c_char,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetReferenceSpaceBoundsRect
#[no_mangle]
pub unsafe extern "system" fn xrGetReferenceSpaceBoundsRect(
	_session: Session,
	_reference_space_type: ReferenceSpaceType,
	_bounds: &mut Extent2Df,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetActionStateBoolean
#[no_mangle]
pub unsafe extern "system" fn xrGetActionStateBoolean(
	_session: Session,
	_get_info: &ActionStateGetInfo,
	_state: &mut ActionStateBoolean,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetActionStateFloat
#[no_mangle]
pub unsafe extern "system" fn xrGetActionStateFloat(
	_session: Session,
	_get_info: &ActionStateGetInfo,
	_state: &mut ActionStateFloat,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetActionStateVector2f
#[no_mangle]
pub unsafe extern "system" fn xrGetActionStateVector2f(
	_session: Session,
	_get_info: &ActionStateGetInfo,
	_state: &mut ActionStateVector2f,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetActionStatePose
#[no_mangle]
pub unsafe extern "system" fn xrGetActionStatePose(
	_session: Session,
	_get_info: &ActionStateGetInfo,
	_state: &mut ActionStatePose,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateActionSet
#[no_mangle]
pub unsafe extern "system" fn xrCreateActionSet(
	_instance: Instance,
	_create_info: &ActionSetCreateInfo,
	_action_set: &mut ActionSet,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroyActionSet
#[no_mangle]
pub unsafe extern "system" fn xrDestroyActionSet(_action_set: ActionSet) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateAction
#[no_mangle]
pub unsafe extern "system" fn xrCreateAction(
	_action_set: ActionSet,
	_create_info: &ActionCreateInfo,
	_action: &mut Action,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroyAction
#[no_mangle]
pub unsafe extern "system" fn xrDestroyAction(_action: Action) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrSuggestInteractionProfileBindings
#[no_mangle]
pub unsafe extern "system" fn xrSuggestInteractionProfileBindings(
	_instance: Instance,
	_suggested_bindings: &InteractionProfileSuggestedBinding,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrAttachSessionActionSets
#[no_mangle]
pub unsafe extern "system" fn xrAttachSessionActionSets(
	_session: Session,
	_attach_info: &SessionActionSetsAttachInfo,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetCurrentInteractionProfile
#[no_mangle]
pub unsafe extern "system" fn xrGetCurrentInteractionProfile(
	_session: Session,
	_top_level_user_path: Path,
	_interaction_profile: &mut InteractionProfileState,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrSyncActions
#[no_mangle]
pub unsafe extern "system" fn xrSyncActions(
	_session: Session,
	_sync_info: &ActionsSyncInfo,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateBoundSourcesForAction
#[no_mangle]
pub unsafe extern "system" fn xrEnumerateBoundSourcesForAction(
	_session: Session,
	_enumerate_info: &BoundSourcesForActionEnumerateInfo,
	_source_capacity_input: u32,
	_source_count_output: &mut u32,
	_sources: &mut Path,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetInputSourceLocalizedName
#[no_mangle]
pub unsafe extern "system" fn xrGetInputSourceLocalizedName(
	_session: Session,
	_get_info: &InputSourceLocalizedNameGetInfo,
	_buffer_capacity_input: u32,
	_buffer_count_output: &mut u32,
	_buffer: &mut c_char,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}
