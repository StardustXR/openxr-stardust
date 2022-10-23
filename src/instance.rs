use openxr_sys::StructureType;

use crate::{
	extensions::xrEnumerateInstanceExtensionProperties,
	oxr::{pfn::VoidFunction, Instance, InstanceCreateInfo, InstanceProperties, Version},
	session::{xrCreateSession, xrDestroySession},
	string::{xrResultToString, xrStructureTypeToString},
	system::{xrGetSystem, xrGetSystemProperties},
	util::{string_from_const_char, wrap_oxr_err},
	wip::*,
	xrEnumerateApiLayerProperties, XrResult,
};

#[derive(Debug)]
pub struct StardustInstance {
	app_name: String,
	app_version: u32,
	engine_name: String,
	engine_version: u32,
	api_version: Version,
}
impl StardustInstance {
	pub fn from_oxr<'a>(instance: Instance) -> Result<&'a mut StardustInstance, XrResult> {
		let instance = instance.into_raw();
		if instance == 0 {
			Err(XrResult::ERROR_HANDLE_INVALID)
		} else {
			Ok(unsafe { &mut *(instance as *mut StardustInstance) })
		}
	}
	pub fn get_proc_addr(&self, name: &str) -> Result<VoidFunction, XrResult> {
		oxr_fns![
			name,
			xrEnumerateInstanceExtensionProperties,
			xrEnumerateApiLayerProperties,
			xrCreateInstance,
			xrDestroyInstance,
			xrResultToString,
			xrStructureTypeToString,
			xrGetInstanceProperties,
			xrGetSystem,
			xrGetSystemProperties,
			xrCreateSession,
			xrDestroySession,
			xrDestroySpace,
			xrEnumerateSwapchainFormats,
			xrCreateSwapchain,
			xrDestroySwapchain,
			xrEnumerateSwapchainImages,
			xrAcquireSwapchainImage,
			xrWaitSwapchainImage,
			xrReleaseSwapchainImage,
			xrBeginSession,
			xrEndSession,
			xrRequestExitSession,
			xrEnumerateReferenceSpaces,
			xrCreateReferenceSpace,
			xrCreateActionSpace,
			xrLocateSpace,
			xrEnumerateViewConfigurations,
			xrEnumerateEnvironmentBlendModes,
			xrGetViewConfigurationProperties,
			xrEnumerateViewConfigurationViews,
			xrBeginFrame,
			xrLocateViews,
			xrEndFrame,
			xrWaitFrame,
			xrApplyHapticFeedback,
			xrStopHapticFeedback,
			xrPollEvent,
			xrStringToPath,
			xrPathToString,
			xrGetReferenceSpaceBoundsRect,
			xrGetActionStateBoolean,
			xrGetActionStateFloat,
			xrGetActionStateVector2f,
			xrGetActionStatePose,
			xrCreateActionSet,
			xrDestroyActionSet,
			xrCreateAction,
			xrDestroyAction,
			xrSuggestInteractionProfileBindings,
			xrAttachSessionActionSets,
			xrGetCurrentInteractionProfile,
			xrSyncActions,
			xrEnumerateBoundSourcesForAction,
			xrGetInputSourceLocalizedName
		]
	}
}

/// # Safety
/// https://registry.khronos.org/OpenXR/specs/1.0/html/xrspec.html#xrCreateInstance
#[no_mangle]
pub unsafe extern "system" fn xrCreateInstance(
	create_info: &InstanceCreateInfo,
	instance: &mut Instance,
) -> XrResult {
	let stardust_instance = Box::new(StardustInstance {
		app_name: string_from_const_char(create_info.application_info.application_name.as_ptr())
			.unwrap(),
		app_version: create_info.application_info.application_version,
		engine_name: string_from_const_char(create_info.application_info.engine_name.as_ptr())
			.unwrap(),
		engine_version: create_info.application_info.engine_version,
		api_version: create_info.application_info.api_version,
	});
	dbg!(&stardust_instance);
	*instance = Instance::from_raw(Box::into_raw(stardust_instance) as u64);

	XrResult::SUCCESS
}

/// # Safety
/// https://registry.khronos.org/OpenXR/specs/1.0/html/xrspec.html#xrDestroyInstance
#[no_mangle]
pub unsafe extern "system" fn xrDestroyInstance(instance: Instance) -> XrResult {
	wrap_oxr_err(move || {
		drop(Box::from_raw(
			StardustInstance::from_oxr(instance)? as *mut _
		));
		Ok(())
	})
}

/// # Safety
/// https://registry.khronos.org/OpenXR/specs/1.0/html/xrspec.html#xrGetInstanceProperties
#[no_mangle]
unsafe extern "system" fn xrGetInstanceProperties(
	_instance: Instance,
	instance_properties: &mut InstanceProperties,
) -> XrResult {
	wrap_oxr_err(move || {
		instance_properties.ty = StructureType::INSTANCE_PROPERTIES;
		instance_properties.runtime_name.fill(0);
		instance_properties.runtime_name[..12]
			.swap_with_slice(&mut b"Stardust XR\0".map(|b| b as i8));
		instance_properties.runtime_version = Version::new(
			env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
			env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
			env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
		);
		Ok(())
	})
}
