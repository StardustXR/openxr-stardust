use crate::{
	instance::StardustInstance,
	oxr::{Instance, Session, SessionCreateInfo},
	util::{get_next_chain, Handle},
	XrResult,
};
use openxr_sys::SystemId;

impl Handle for Session {
	type StardustType = StardustSession;

	fn raw(&self) -> u64 {
		self.into_raw()
	}
}

pub struct StardustSession {
	instance: Instance,
	node_path: String,
}
impl StardustSession {
	fn new(instance: Instance, system: SystemId) -> Result<Self, XrResult> {
		let id = nanoid::nanoid!();
		{
			let instance = instance.get_stardust()?;
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
	pub fn instance<'a>(&'a mut self) -> Result<&'a mut StardustInstance, XrResult> {
		self.instance.get_stardust()
	}
	pub fn node_path(&self) -> &str {
		&self.node_path
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
		let instance = oxr_instance.get_stardust()?;
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
		session.destroy()?;
	}
}
