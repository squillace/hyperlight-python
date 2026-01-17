use hyperlight_host::{MultiUseSandbox, Result, sandbox::snapshot::Snapshot};

use crate::sandbox::PySandbox;

/// Loaded Python sandbox for executing Python code.
/// This sandbox has the Python runtime loaded and initialized and it allows
/// running Python scripts in an isolated environment.
pub struct LoadedPySandbox {
    /// Inner multi-use sandbox
    inner: MultiUseSandbox,
    /// Snapshot of the initial state before loading the Python runtime
    snapshot: Snapshot,
}

impl LoadedPySandbox {
    /// Create a new [`LoadedPySandbox`]
    /// # Arguments
    /// * `inner` - The inner multi-use sandbox with the Python runtime loaded
    /// * `snapshot` - The snapshot of the initial state before loading the Python runtime
    pub(super) fn new(inner: MultiUseSandbox, snapshot: Snapshot) -> Result<LoadedPySandbox> {
        Ok(LoadedPySandbox { inner, snapshot })
    }

    /// Returns whether the sandbox is poisoned.
    /// A poisoned sandbox indicates that a previous operation has failed
    /// and the sandbox is no longer in a valid state for further operations.
    pub fn poisoned(&self) -> bool {
        self.inner.poisoned()
    }

    /// Run a Python script in the sandbox.
    /// # Arguments
    /// * `code` - The Python code to execute as a string
    /// # Returns
    /// * `Result<bool>` - Returns `Ok(true)` if the script executed successfully, otherwise
    /// returns an error.
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
    ///     let code = r#"
    /// def greet(name):
    ///     return f"Hello, {name}!"
    ///
    /// result = greet("World")
    /// "#.to_string();
    ///
    ///     let sandbox = proto_sbox.load_runtime()?;
    ///     let mut sandbox = sandbox.get_loaded_sandbox()?;
    ///
    ///     let success = sandbox.run_script(code)?;
    ///     assert!(success);
    ///     Ok(())
    /// }
    /// ```
    pub fn run_script(&mut self, code: String) -> Result<bool> {
        self.inner.call("exec_python", code)
    }

    /// Unload the Python runtime and return to a [`PySandbox`].
    /// This means that the Python runtime is no longer initialized in the sandbox
    /// and it cannot run Python scripts until it is loaded again.
    ///
    /// # Returns
    /// * `Result<PySandbox>` - The unloaded Python sandbox.
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
    ///     let py_sbox = sandbox.unload()?;
    ///     Ok(())
    /// }
    /// ```
    pub fn unload(self) -> Result<PySandbox> {
        PySandbox::from_loaded(self.inner, self.snapshot)
    }
}
