use crate::XrResult;
use std::ffi::{c_char, CStr};

/// # Safety
/// https://registry.khronos.org/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateInstanceExtensionProperties
pub unsafe fn enumerate<I: Clone>(
	input_count: u32,
	output_count: &mut Option<u32>,
	items_ptr: *mut I,
	items: &[I],
) -> XrResult {
	if output_count.is_none() {
		return XrResult::ERROR_VALIDATION_FAILURE;
	}
	*output_count = Some(items.len() as u32);
	if input_count == 0 || items_ptr.is_null() {
		return XrResult::SUCCESS;
	}
	if input_count < items.len() as u32 {
		return XrResult::ERROR_SIZE_INSUFFICIENT;
	}
	if items_ptr.is_null() {
		return XrResult::SUCCESS;
	}
	std::ptr::copy_nonoverlapping(items.as_ptr(), items_ptr, items.len());

	XrResult::SUCCESS
}

macro_rules! oxr_fns {
	($s:expr,$($f:ident),*) => {
		match $s {
			$(
				stringify!($f) => Ok(unsafe { std::mem::transmute($f as usize) }),
			)*
			_ => Err(XrResult::ERROR_HANDLE_INVALID),
		}
	};
}

pub fn str_from_const_char<'a>(ptr: *const c_char) -> Result<&'a str, XrResult> {
	if ptr.is_null() {
		return Err(XrResult::ERROR_VALIDATION_FAILURE);
	}
	Ok(unsafe {
		CStr::from_ptr(ptr)
			.to_str()
			.map_err(|_| XrResult::ERROR_VALIDATION_FAILURE)?
	})
}

pub fn wrap_oxr_fn<F: FnOnce() -> Result<(), XrResult>>(f: F) -> XrResult {
	match f() {
		Ok(_) => XrResult::SUCCESS,
		Err(e) => e,
	}
}

macro_rules! wrap_oxr {
	($($b:tt)+) => {
		$crate::util::wrap_oxr_fn(move || -> std::result::Result<(), XrResult> { $($b)* })
	};
}
