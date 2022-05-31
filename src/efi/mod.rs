use uefi::{table::{SystemTable, Runtime}, Status};

// Contains the startup boot code (and tests)
pub mod boot;
pub mod runtime;
pub mod proto;
pub mod acpi;

// Taken from the uefi-test-runner
pub fn check_revision(rev: uefi::table::Revision) {
    let (major, minor) = (rev.major(), rev.minor());

    info!("UEFI {}.{}", major, minor / 10);

    assert!(major >= 2, "Running on an old, unsupported version of UEFI");
    assert!(
        minor >= 30,
        "Old version of UEFI 2, some features might not be available."
    );
}

// Taken from the uefi-test-runner
pub fn shutdown(mut st: SystemTable<Runtime>) -> ! {
    // Get our text output back.
    // st.stdout().reset(false).unwrap();

    // Inform the user, and give him time to read on real hardware
    // info!("Testing complete, shutting down in 3 seconds...");
    // st.stall(3_000_000);

    use uefi::table::runtime::ResetType;

    // Shut down the system
    let rt = unsafe { st.runtime_services() };
    rt.reset(ResetType::Shutdown, Status::SUCCESS, None);
}
