use crate::{HostPrintFn, sandbox::PySandbox};
use hyperlight_host::{GuestBinary, Result, UninitializedSandbox, sandbox::SandboxConfiguration};

/// Sandbox for initializing a Python runtime.
/// This sandbox does not have the Python runtime loaded yet.
/// Use [`ProtoPySandbox::load_runtime`] to load the Python runtime and obtain a [`PySandbox`].
/// # Example
/// ```no_run
/// use hyperlight_python::sandbox::ProtoPySandbox;
/// use hyperlight_python::sandbox::SandboxBuilder;
///
/// fn main() -> hyperlight_host::Result<()> {
///     let mut proto_sandbox = SandboxBuilder::new()
///         .build()?;
///
///     let py_sandbox = proto_sandbox.load_runtime()?;
///
///     // Now you can use `py_sandbox` to run Python code.
///     let mut loaded = py_sandbox.get_loaded_sandbox()?;
///     let success = loaded.run_script("print('Hello from Python!')".to_string())?;
///     assert!(success);
///
///     Ok(())
/// }
/// ```
pub struct ProtoPySandbox {
    /// Inner uninitialized sandbox
    inner: UninitializedSandbox,
}

impl ProtoPySandbox {
    /// Create a new [`ProtoPySandbox`]
    ///
    /// # Arguments
    /// * `guest_binary` - The guest binary to use for the sandbox
    /// * `cfg` - Optional configuration for the sandbox
    /// * `host_print_writer` - Optional host print function
    ///
    /// # Errors
    /// Returns an error if the sandbox could not be created
    pub(super) fn new(
        guest_binary: GuestBinary,
        cfg: Option<SandboxConfiguration>,
        host_print_writer: Option<HostPrintFn>,
    ) -> Result<Self> {
        let mut usbox: UninitializedSandbox = UninitializedSandbox::new(guest_binary, cfg)?;

        if let Some(host_print_writer) = host_print_writer {
            usbox.register_print(host_print_writer)?;
        }

        Ok(Self { inner: usbox })
    }

    /// Load the Python runtime into the sandbox.
    /// This initializes the Python runtime and returns a [`PySandbox`].
    /// # Errors
    /// Returns an error if the Python runtime could not be initialized.
    pub fn load_runtime(self) -> Result<PySandbox> {
        let multi_use_sandbox = self.inner.evolve()?;

        PySandbox::new(multi_use_sandbox)
    }
}
