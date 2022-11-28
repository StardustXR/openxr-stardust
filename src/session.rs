use crate::{
	instance::StardustInstance,
	oxr::{Instance, Session, SessionCreateInfo},
	util::get_next_chain,
	XrResult,
};
use openxr_sys::SystemId;

pub struct StardustSession {
	instance: Instance,
	node_path: String,
}
impl StardustSession {
	fn new(instance: Instance, system: SystemId) -> Result<Self, XrResult> {
		let id = nanoid::nanoid!();
		{
			let instance = StardustInstance::from_oxr(instance)?;
			instance.send_signal(
				&format!("/openxr/system{}", system.into_raw()),
				"create_session",
				&id,
			)?;
		}

		let session = StardustSession {
			instance,
			node_path: format!("/openxr/system{}/{}", system.into_raw(), id),
		};
		Ok(session)
	}
	pub fn from_oxr<'a>(session: Session) -> Result<&'a mut StardustSession, XrResult> {
		let instance = session.into_raw();
		if instance == 0 {
			Err(XrResult::ERROR_HANDLE_INVALID)
		} else {
			Ok(unsafe { &mut *(instance as *mut StardustSession) })
		}
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateSession
#[no_mangle]
pub unsafe extern "system" fn xrCreateSession(
	oxr_instance: Instance,
	create_info: &SessionCreateInfo,
	session: &mut Session,
) -> XrResult {
	wrap_oxr! {
		let instance = StardustInstance::from_oxr(oxr_instance)?;
		let next_chain = get_next_chain(create_info);
		let contains_graphics = next_chain.iter().any(|next| format!("{:?}", next.ty).contains("_enable")); // another ugly hack
		if !contains_graphics && !instance.extension_headless_enabled {
			Err(XrResult::ERROR_GRAPHICS_REQUIREMENTS_CALL_MISSING)?;
		}

		let stardust_session = Box::new(StardustSession::new(oxr_instance, create_info.system_id)?);
		*session = Session::from_raw(Box::into_raw(stardust_session) as u64);
	}
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroySession
#[no_mangle]
pub unsafe extern "system" fn xrDestroySession(session: Session) -> XrResult {
	wrap_oxr! {
		drop(Box::from_raw(
			StardustSession::from_oxr(session)? as *mut _
		));
	}
}
