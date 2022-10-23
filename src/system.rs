use crate::{
	oxr::{Instance, SystemGetInfo, SystemId, SystemProperties},
	util::wrap_oxr_err,
	XrResult,
};

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetSystem
#[no_mangle]
pub unsafe extern "system" fn xrGetSystem(
	_instance: Instance,
	_get_info: &SystemGetInfo,
	_system_id: &mut SystemId,
) -> XrResult {
	wrap_oxr_err(move || {
		todo!();
	})
}

/// # Safety
/// https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#xrGetSystemProperties
#[no_mangle]
pub unsafe extern "system" fn xrGetSystemProperties(
	_instance: Instance,
	_system_id: SystemId,
	_properties: &mut SystemProperties,
) -> XrResult {
	wrap_oxr_err(move || {
		todo!();
	})
}
