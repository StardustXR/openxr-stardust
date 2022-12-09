use crate::{
	instance::StardustInstance,
	util::{str_from_const_char, Handle},
	XrResult,
};
use openxr_sys::{
	Action, ActionCreateInfo, ActionSet, ActionSetCreateInfo, ActionStateBoolean, ActionStateFloat,
	ActionStateGetInfo, ActionStatePose, ActionStateVector2f, ActionsSyncInfo,
	BoundSourcesForActionEnumerateInfo, HapticActionInfo, HapticBaseHeader,
	InputSourceLocalizedNameGetInfo, Instance, InteractionProfileState,
	InteractionProfileSuggestedBinding, Path, Session, SessionActionSetsAttachInfo,
};
use std::{ffi::c_char, ptr::slice_from_raw_parts};

impl Handle for ActionSet {
	type StardustType = StardustActionSet;

	fn raw(&self) -> u64 {
		self.into_raw()
	}
}

pub struct StardustActionSet {
	instance: Instance,
	node_path: String,
}
impl StardustActionSet {
	fn new(instance: Instance, create_info: &ActionSetCreateInfo) -> Result<Self, XrResult> {
		let stardust_instance = instance.get_stardust()?;
		let name = str_from_const_char(create_info.action_set_name.as_ptr())?;
		let localized_name = str_from_const_char(create_info.localized_action_set_name.as_ptr())?;
		stardust_instance.send_signal(
			"/openxr",
			"create_action_set",
			&(name, localized_name, create_info.priority),
		)?;

		let action_set = StardustActionSet {
			instance,
			node_path: format!("/openxr/action_set/{}", name),
		};
		Ok(action_set)
	}
	pub fn instance<'a>(&'a mut self) -> Result<&'a mut StardustInstance, XrResult> {
		self.instance.get_stardust()
	}
	pub fn node_path(&self) -> &str {
		&self.node_path
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateActionSet
#[no_mangle]
pub unsafe extern "system" fn xrCreateActionSet(
	instance: Instance,
	create_info: &ActionSetCreateInfo,
	action_set: &mut ActionSet,
) -> XrResult {
	wrap_oxr! {
		// let next_chain = get_next_chain(create_info);

		let stardust_action_set = Box::new(StardustActionSet::new(instance, create_info)?);
		*action_set = ActionSet::from_raw(Box::into_raw(stardust_action_set) as u64);
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroyActionSet
#[no_mangle]
pub unsafe extern "system" fn xrDestroyActionSet(action_set: ActionSet) -> XrResult {
	wrap_oxr! {
		action_set.destroy()?;
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

impl Handle for Action {
	type StardustType = StardustAction;

	fn raw(&self) -> u64 {
		self.into_raw()
	}
}

pub struct StardustAction {
	action_set: ActionSet,
	node_path: String,
}
impl StardustAction {
	fn new(action_set: ActionSet, create_info: &ActionCreateInfo) -> Result<Self, XrResult> {
		let stardust_action_set = action_set.get_stardust()?;
		let stardust_action_set_node_path = stardust_action_set.node_path.clone();
		let stardust_instance = stardust_action_set.instance()?;
		let name = str_from_const_char(create_info.action_name.as_ptr())?;
		let localized_name = str_from_const_char(create_info.localized_action_name.as_ptr())?;
		stardust_instance.send_signal(
			&stardust_action_set_node_path,
			"create_action",
			&(name, localized_name),
		)?;

		let action = StardustAction {
			action_set,
			node_path: format!("{}/{}", stardust_action_set_node_path, name),
		};
		Ok(action)
	}
	pub fn action_set<'a>(&'a mut self) -> Result<&'a mut StardustActionSet, XrResult> {
		self.action_set.get_stardust()
	}
	pub fn node_path(&self) -> &str {
		&self.node_path
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateAction
#[no_mangle]
pub unsafe extern "system" fn xrCreateAction(
	action_set: ActionSet,
	create_info: &ActionCreateInfo,
	action: &mut Action,
) -> XrResult {
	wrap_oxr! {
		// let next_chain = get_next_chain(create_info);

		let stardust_action = Box::new(StardustAction::new(action_set, create_info)?);
		*action = Action::from_raw(Box::into_raw(stardust_action) as u64);
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroyAction
#[no_mangle]
pub unsafe extern "system" fn xrDestroyAction(action: Action) -> XrResult {
	wrap_oxr! {
		action.destroy()?;
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrSuggestInteractionProfileBindings
#[no_mangle]
pub unsafe extern "system" fn xrSuggestInteractionProfileBindings(
	instance: Instance,
	suggested_bindings: &InteractionProfileSuggestedBinding,
) -> XrResult {
	wrap_oxr! {
		let stardust_instance = instance.get_stardust()?;
		let suggested_bindings = slice_from_raw_parts(suggested_bindings.suggested_bindings, suggested_bindings.count_suggested_bindings as usize);
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
