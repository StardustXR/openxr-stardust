use crate::{
	extensions::xrEnumerateInstanceExtensionProperties,
	session::{xrCreateSession, xrDestroySession},
	string::{xrResultToString, xrStructureTypeToString},
	system::{xrGetSystem, xrGetSystemProperties},
	util::{copy_str_to_buffer, str_from_const_char},
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

pub struct StardustInstance {
	runtime: Runtime,
	message_sender: MessageSender,
}
impl StardustInstance {
	pub fn new(info: &InstanceCreateInfo) -> Result<Self, XrResult> {
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
		};

		#[derive(Default, Serialize)]
		struct InstanceSetupInfo {
			app_name: String,
			app_version: u32,
			engine_name: String,
			engine_version: u32,
			api_version: u64,
		}
		let info = InstanceSetupInfo {
			app_name: str_from_const_char(info.application_info.application_name.as_ptr())?
				.to_string(),
			app_version: info.application_info.application_version,
			engine_name: str_from_const_char(info.application_info.engine_name.as_ptr())?
				.to_string(),
			engine_version: info.application_info.engine_version,
			api_version: info.application_info.api_version.into_raw(),
		};
		instance.send_signal("/openxr", "setupInstance", &info)?;

		Ok(instance)
	}
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
	pub fn send_signal<S: Serialize>(
		&mut self,
		node_path: &str,
		signal_name: &str,
		data: &S,
	) -> Result<(), XrResult> {
		self.runtime
			.block_on(
				self.message_sender
					.signal(node_path, signal_name, &serialize(data).unwrap()),
			)
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
	create_info: &InstanceCreateInfo,
	instance: &mut Instance,
) -> XrResult {
	wrap_oxr! {
		let stardust_instance = Box::new(StardustInstance::new(create_info)?);
		*instance = Instance::from_raw(Box::into_raw(stardust_instance) as u64);
		Ok(())
	}
}

/// # Safety
/// https://registry.khronos.org/OpenXR/specs/1.0/html/xrspec.html#xrDestroyInstance
#[no_mangle]
pub unsafe extern "system" fn xrDestroyInstance(instance: Instance) -> XrResult {
	wrap_oxr! {
		drop(Box::from_raw(
			StardustInstance::from_oxr(instance)? as *mut _
		));
		Ok(())
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
		Ok(())
	}
}
