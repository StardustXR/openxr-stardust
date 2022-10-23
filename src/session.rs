use crate::{
	oxr::{Instance, Session, SessionCreateInfo},
	util::wrap_oxr_err,
	XrResult,
};

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrCreateSession
#[no_mangle]
pub unsafe extern "system" fn xrCreateSession(
	_instance: Instance,
	_create_info: &SessionCreateInfo,
	_session: &mut Session,
) -> XrResult {
	wrap_oxr_err(move || {
		todo!();
	})
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrDestroySession
#[no_mangle]
pub unsafe extern "system" fn xrDestroySession(_session: Session) -> XrResult {
	wrap_oxr_err(move || {
		todo!();
	})
}
