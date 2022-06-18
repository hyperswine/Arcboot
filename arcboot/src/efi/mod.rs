use uefi::{table::{SystemTable, Runtime, runtime::ResetType}, Status};

// Contains the startup boot code (and tests)
pub mod boot;
pub mod runtime;
pub mod proto;
pub mod acpi;

pub fn check_revision(rev: uefi::table::Revision) {
    let (major, minor) = (rev.major(), rev.minor());

    info!("UEFI {}.{}", major, minor / 10);

    assert!(major >= 2, "Running on an old, unsupported version of UEFI");
    assert!(
        minor >= 30,
        "Old version of UEFI 2, some features might not be available."
    );
}

pub fn shutdown(mut st: SystemTable<Runtime>) -> ! {
    // Shut down the system
    let rt = unsafe { st.runtime_services() };
    rt.reset(ResetType::Shutdown, Status::SUCCESS, None);
}
