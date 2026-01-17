mod loaded_py_sandbox;
mod proto_py_sandbox;
mod py_sandbox;
mod sandbox_builder;

pub use loaded_py_sandbox::LoadedPySandbox;
pub use proto_py_sandbox::ProtoPySandbox;
pub use py_sandbox::PySandbox;
pub use sandbox_builder::SandboxBuilder;

// This include! macro is replaced by the build.rs script.
// The build.rs script reads the jshost.exe binary into a static byte array named PYTHONHOST.
include!(concat!(env!("OUT_DIR"), "/host_resource.rs"));
