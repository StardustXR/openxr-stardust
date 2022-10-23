use crate::{
	oxr::{FormFactor, Instance, SystemGetInfo, SystemId, SystemProperties},
	XrResult,
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
	_system_id: SystemId,
	_properties: &mut SystemProperties,
) -> XrResult {
	wrap_oxr! {
		todo!();
	}
}
