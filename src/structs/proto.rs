#[cfg(not(feature = "regen_proto"))]
include!("../chromeos_update_engine.rs");

#[cfg(feature = "regen_proto")]
include!(concat!(env!("OUT_DIR"), "/chromeos_update_engine.rs"));
