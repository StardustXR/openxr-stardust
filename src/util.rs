use openxr_sys::LoaderInitInfoBaseHeaderKHR;

use crate::XrResult;
use std::{
	ffi::{c_char, CStr},
	ptr,
};

pub type XrRsResult = Result<(), XrResult>;

macro_rules! wrap_oxr {
	($($b:tt)+) => {
		#[allow(unreachable_code)]
		match (move || -> std::result::Result<(), openxr_sys::Result> { {$($b)*} Ok(()) })() {
			Ok(_) => openxr_sys::Result::SUCCESS,
			Err(e) => e,
		}
	};
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

/// # Safety
/// https://registry.khronos.org/OpenXR/specs/1.0/html/xrspec.html#xrEnumerateInstanceExtensionProperties
pub unsafe fn enumerate<I: Clone>(
	input_count: u32,
	output_count: &mut Option<u32>,
	items_ptr: *mut I,
	items: &[I],
) -> Result<(), XrResult> {
	// if output_count.is_none() {
	// 	return Err(XrResult::ERROR_VALIDATION_FAILURE);
	// }
	*output_count = Some(items.len() as u32);
	if input_count == 0 || items_ptr.is_null() {
		return Ok(());
	}
	if input_count < items.len() as u32 {
		return Err(XrResult::ERROR_SIZE_INSUFFICIENT);
	}
	if items_ptr.is_null() {
		return Ok(());
	}
	std::ptr::copy_nonoverlapping(items.as_ptr(), items_ptr, items.len());

	Ok(())
}

pub fn str_from_const_char<'a>(ptr: *const c_char) -> Result<&'a str, XrResult> {
	if ptr.is_null() {
		return Err(XrResult::ERROR_VALIDATION_FAILURE);
	}

	unsafe { CStr::from_ptr(ptr) }
		.to_str()
		.map_err(|_| XrResult::ERROR_VALIDATION_FAILURE)
}

pub fn copy_str_to_buffer(string: &str, buf: &mut [c_char]) {
	bytemuck::cast_slice_mut(&mut buf[..string.len()]).copy_from_slice(string.as_bytes());
	buf[string.len()] = 0;
}

pub unsafe fn get_next_chain<F>(first: &F) -> Vec<LoaderInitInfoBaseHeaderKHR> {
	// really gotta improve this tbh

	let mut chain: Vec<LoaderInitInfoBaseHeaderKHR> =
		vec![std::mem::transmute::<&F, &LoaderInitInfoBaseHeaderKHR>(first).clone()];
	while let Some(next) = chain
		.last()
		.and_then(|c| (c.next != ptr::null()).then_some(c.next))
	{
		chain.push((&*std::mem::transmute::<_, *const LoaderInitInfoBaseHeaderKHR>(next)).clone());
	}
	chain
}
