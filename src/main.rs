extern crate windows_service;

use std::ffi::OsString;
use std::time::Duration;
use windows_service::service_dispatcher;
use windows_service::define_windows_service;
use windows_service::service::{
    ServiceControl,
    ServiceStatus,
    ServiceControlAccept,
    ServiceExitCode,
    ServiceState, 
    ServiceType,
};
use windows_service::service_control_handler::{self, ServiceControlHandlerResult};

define_windows_service!(ffi_service_main, service_main);

fn service_main(arguments: Vec<OsString>) {
    if let Err(_e) = run_service(arguments) {
        // Handle errors in some way... or not...
    }
}

fn run_service(_arguments: Vec<OsString>) -> Result<(), windows_service::Error> {
    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
            ServiceControl::Stop | ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };

    // Register system service event handler.
    let status_handle = service_control_handler::register("totally_spies", event_handler)?;

    let next_status = ServiceStatus {
        // Should match the one from system service registry
        service_type: ServiceType::OWN_PROCESS,

        // The new state
        current_state: ServiceState::Running,

        // Accept stop events when running
        controls_accepted: ServiceControlAccept::STOP,

        // Used to report an error when starting or stopping only, otherwise must be zero
        exit_code: ServiceExitCode::Win32(0),

        // Only used for pending states, otherwise must be zero
        checkpoint: 0,
        wait_hint: Duration::default(),

        // Process ID
        process_id: None,
    };

    // Tell the system that the service is running now
    status_handle.set_service_status(next_status)?;

    Ok(())
}

fn main() -> Result<(), windows_service::Error> {
    service_dispatcher::start("totally_spies", ffi_service_main)?;
    Ok(())
}