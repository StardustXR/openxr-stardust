use crate::{
	extensions::xrEnumerateInstanceExtensionProperties,
	input::{
		xrApplyHapticFeedback, xrAttachSessionActionSets, xrCreateAction, xrCreateActionSet,
		xrDestroyAction, xrDestroyActionSet, xrEnumerateBoundSourcesForAction,
		xrGetActionStateBoolean, xrGetActionStateFloat, xrGetActionStatePose,
		xrGetActionStateVector2f, xrGetCurrentInteractionProfile, xrGetInputSourceLocalizedName,
		xrStopHapticFeedback, xrSuggestInteractionProfileBindings, xrSyncActions,
	},
	session::{xrCreateSession, xrDestroySession},
	string::{xrResultToString, xrStructureTypeToString},
	system::{
		xrEnumerateEnvironmentBlendModes, xrEnumerateViewConfigurationViews,
		xrEnumerateViewConfigurations, xrGetSystem, xrGetSystemProperties,
		xrGetViewConfigurationProperties,
	},
	util::{copy_str_to_buffer, str_from_const_char, Handle},
	wip::*,
	xrEnumerateApiLayerProperties, XrResult,
};
use openxr_sys::{
	pfn::VoidFunction, Instance, InstanceCreateInfo, InstanceProperties, StructureType, Version,
};
use serde::{de::DeserializeOwned, Serialize};
use stardust_xr::{
	client,
	messenger::{self, MessageSender},
	scenegraph::{Scenegraph, ScenegraphError},
	schemas::flex::{deserialize, serialize},
};
use std::ptr::slice_from_raw_parts;
use tokio::runtime::Runtime;

struct DummyScenegraph;
impl Scenegraph for DummyScenegraph {
	fn execute_method(
		&self,
		_path: &str,
		_method: &str,
		_data: &[u8],
	) -> Result<Vec<u8>, stardust_xr::scenegraph::ScenegraphError> {
		Err(ScenegraphError::NodeNotFound)
	}
}

#[derive(Default, Serialize)]
struct SetupInfo {
	app_info: ApplicationInfo,
	extension_names: Vec<String>,
}
#[derive(Default, Serialize)]
struct ApplicationInfo {
	app_name: String,
	app_version: u32,
	engine_name: String,
	engine_version: u32,
	api_version: u64,
}

impl Handle for Instance {
	type StardustType = StardustInstance;

	fn raw(&self) -> u64 {
		self.into_raw()
	}
}

pub struct StardustInstance {
	runtime: Runtime,
	message_sender: MessageSender,
	pub extension_headless_enabled: bool,
}
impl StardustInstance {
	fn new(info: &SetupInfo) -> Result<Self, XrResult> {
		let runtime = tokio::runtime::Builder::new_current_thread()
			.enable_io()
			.enable_time()
			.build()
			.map_err(|_| XrResult::ERROR_RUNTIME_UNAVAILABLE)?;
		let client = runtime
			.block_on(client::connect())
			.map_err(|_| XrResult::ERROR_RUNTIME_UNAVAILABLE)?;
		let (message_sender, mut message_receiver) = messenger::create(client);
		runtime.spawn(
			async move { while message_receiver.dispatch(&DummyScenegraph).await.is_ok() {} },
		);

		let mut instance = StardustInstance {
			runtime,
			message_sender,
			extension_headless_enabled: info.extension_names.iter().any(|n| n == "XR_MND_headless"),
		};
		instance.send_signal("/openxr", "setup_instance", &info)?;

		Ok(instance)
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
			xrEnumerateViewConfigurationViews,
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
	pub fn send_signal<S: Serialize>(
		&mut self,
		node_path: &str,
		signal_name: &str,
		data: &S,
	) -> Result<(), XrResult> {
		let serialized_data = serialize(data).map_err(|_| XrResult::ERROR_RUNTIME_FAILURE)?;
		let signal_future = self
			.message_sender
			.signal(node_path, signal_name, &serialized_data);
		self.runtime
			.block_on(signal_future)
			.map_err(|_| XrResult::ERROR_RUNTIME_FAILURE)
	}
	pub fn execute_method<S: Serialize, D: DeserializeOwned>(
		&mut self,
		node_path: &str,
		method_name: &str,
		send_data: &S,
	) -> Result<anyhow::Result<D>, XrResult> {
		let send_data = serialize(send_data).map_err(|_| XrResult::ERROR_RUNTIME_FAILURE)?;
		let execute_method_future = self
			.message_sender
			.method(node_path, method_name, &send_data);

		let future = async move {
			let timeout = tokio::time::sleep(core::time::Duration::from_secs(1));
			Ok(tokio::select! {
				_ = timeout => return Err(XrResult::ERROR_RUNTIME_FAILURE),
				d = execute_method_future => {
					let data = d.map_err(|_| XrResult::ERROR_RUNTIME_FAILURE)?;
					data.and_then(|data| Ok(deserialize(&data)?))
				}
			})
		};
		self.runtime.block_on(future)
	}
}

/// # Safety
/// https://registry.khronos.org/OpenXR/specs/1.0/html/xrspec.html#xrCreateInstance
#[no_mangle]
pub unsafe extern "system" fn xrCreateInstance(
	info: &InstanceCreateInfo,
	instance: &mut Instance,
) -> XrResult {
	wrap_oxr! {
		let app_name = str_from_const_char(info.application_info.application_name.as_ptr())?
		.to_string();
		let engine_name = str_from_const_char(info.application_info.engine_name.as_ptr())?
		.to_string();
		let extension_names: Result<Vec<String>, XrResult> =
			(&*slice_from_raw_parts(info.enabled_extension_names, info.enabled_extension_count as usize))
				.iter()
				.map(|name| str_from_const_char(&**name).map(String::from)).collect();
				let extension_names = extension_names.map_err(|_| XrResult::ERROR_VALIDATION_FAILURE)?;

		println!("Extensions: {:#?}", extension_names);
		let info = SetupInfo {
			app_info: ApplicationInfo {
				app_name,
				app_version: info.application_info.application_version,
				engine_name,
				engine_version: info.application_info.engine_version,
				api_version: info.application_info.api_version.into_raw(),
			},
			extension_names,
		};

		let stardust_instance = Box::new(StardustInstance::new(&info)?);
		*instance = Instance::from_raw(Box::into_raw(stardust_instance) as u64);
	}
}

/// # Safety
/// https://registry.khronos.org/OpenXR/specs/1.0/html/xrspec.html#xrDestroyInstance
#[no_mangle]
pub unsafe extern "system" fn xrDestroyInstance(instance: Instance) -> XrResult {
	wrap_oxr! {
		instance.destroy()?;
	}
}

/// # Safety
/// https://registry.khronos.org/OpenXR/specs/1.0/html/xrspec.html#xrGetInstanceProperties
#[no_mangle]
pub unsafe extern "system" fn xrGetInstanceProperties(
	_instance: Instance,
	instance_properties: &mut InstanceProperties,
) -> XrResult {
	wrap_oxr! {
		instance_properties.ty = StructureType::INSTANCE_PROPERTIES;
		copy_str_to_buffer("Stardust XR", &mut instance_properties.runtime_name);
		instance_properties.runtime_version = Version::new(
			env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
			env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
			env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
		);
	}
}
