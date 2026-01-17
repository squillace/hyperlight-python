use hyperlight_host::sandbox::snapshot::Snapshot;
use hyperlight_host::{MultiUseSandbox, Result, new_error};

use crate::sandbox::LoadedPySandbox;

/// Python sandbox without the Python runtime loaded.
/// This sandbox allows initializing the Python runtime and obtaining a [`LoadedPySandbox`]
/// for executing Python scripts.
///
/// # Example
/// ```
/// use hyperlight_python::sandbox::PySandbox;
/// use hyperlight_python::sandbox::SandboxBuilder;
///
/// fn main() -> hyperlight_host::Result<()> {
///     let mut proto_sbox = SandboxBuilder::new()
///         .build()?;
///
///     let sandbox = proto_sbox.load_runtime()?;
///     let mut sandbox = sandbox.get_loaded_sandbox()?;
///
///     let code = r#"
/// def greet(name):
///     return f"Hello, {name}!"
///
/// result = greet("World")
/// "#.to_string();
///
///     let success = sandbox.run_script(code)?;
///     assert!(success);
///
///     Ok(())
/// }
/// ```
pub struct PySandbox {
    /// Inner multi-use sandbox
    pub(super) inner: MultiUseSandbox,
    /// Snapshot of the initial state
    snapshot: Snapshot,
}

impl PySandbox {
    /// Create a new [`PySandbox`] from a [`MultiUseSandbox`]
    /// Creates a snapshot of the initial state for resetting purposes.
    ///
    /// # Arguments
    /// * `inner` - The inner multi-use sandbox
    pub(super) fn new(mut inner: MultiUseSandbox) -> Result<Self> {
        let snapshot = inner.snapshot()?;
        Ok(Self { inner, snapshot })
    }

    /// Create a new [`PySandbox`] from a loaded [`MultiUseSandbox`] and a [`Snapshot`]
    /// Restores the sandbox to the given snapshot.
    /// Used for unloading the Python runtime.
    /// # Arguments
    /// * `inner` - The inner multi-use sandbox
    /// * `snapshot` - The snapshot to restore
    pub(super) fn from_loaded(mut inner: MultiUseSandbox, snapshot: Snapshot) -> Result<Self> {
        inner.restore(&snapshot.clone())?;
        Ok(Self { inner, snapshot })
    }

    /// Initialize the Python runtime and obtain a [`LoadedPySandbox`].
    ///
    /// # Returns
    /// * `Result<LoadedPySandbox>` - The loaded Python sandbox.
    ///
    /// # Errors
    /// Returns an error if the Python runtime could not be initialized.
    pub fn get_loaded_sandbox(mut self) -> Result<LoadedPySandbox> {
        self.inner
            .call::<bool>("init_python", ())
            .map_err(|e| new_error!("Could not initialize Python runtime: {:?}", e))?;

        LoadedPySandbox::new(self.inner, self.snapshot)
    }

    /// Returns whether the sandbox is poisoned.
    /// A poisoned sandbox indicates that a previous operation has failed
    /// and the sandbox is no longer in a valid state for further operations.
    pub fn poisoned(&self) -> bool {
        self.inner.poisoned()
    }
}
