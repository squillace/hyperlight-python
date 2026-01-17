use hyperlight_host::GuestBinary;
use hyperlight_host::HyperlightError;
use hyperlight_host::Result;
use hyperlight_host::is_hypervisor_present;
use hyperlight_host::sandbox::SandboxConfiguration;
#[cfg(feature = "gdb")]
use hyperlight_host::sandbox::config::DebugInfo;

use crate::HostPrintFn;
use crate::sandbox::proto_py_sandbox::ProtoPySandbox;

/// Sandbox builder for the [`ProtoPySandbox`]
pub struct SandboxBuilder {
    /// Configuration for the inner sandbox
    cfg: SandboxConfiguration,
    /// Optional host print function
    host_print_fn: Option<HostPrintFn>,
}

impl SandboxBuilder {
    /// Create a new [`SandboxBuilder`] with default configuration
    /// The default configuration sets the stack size to 128 kB and the heap size to 512 kB
    ///
    /// # Example
    /// ```
    /// use hyperlight_python::sandbox::SandboxBuilder;
    ///
    /// fn main() -> hyperlight_host::Result<()> {
    ///     let sandbox = SandboxBuilder::new().build()?;
    ///     Ok(())
    /// }
    /// ```
    pub fn new() -> Self {
        let mut cfg = SandboxConfiguration::default();
        cfg.set_stack_size(128 * 1024);
        cfg.set_heap_size(512 * 1024);

        Self {
            cfg,
            host_print_fn: None,
        }
    }

    /// Set the stack size for the sandbox
    /// # Arguments
    /// * `size` - Size of the stack in bytes
    ///
    /// # Example
    /// ```
    /// use hyperlight_python::sandbox::SandboxBuilder;
    ///
    /// fn main() -> hyperlight_host::Result<()> {
    ///     let sandbox = SandboxBuilder::new()
    ///         .with_stack_size(256 * 1024) // 256 kB
    ///         .build()?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn with_stack_size(mut self, size: u64) -> Self {
        self.cfg.set_stack_size(size);

        self
    }

    /// Set the heap size for the sandbox
    /// # Arguments
    /// * `size` - Size of the heap in bytes
    ///
    /// # Example
    /// ```
    /// use hyperlight_python::sandbox::SandboxBuilder;
    ///
    /// fn main() -> hyperlight_host::Result<()> {
    ///     let sandbox = SandboxBuilder::new()
    ///         .with_heap_size(1 * 1024 * 1024) // 1 MB
    ///         .build()?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn with_heap_size(mut self, size: u64) -> Self {
        self.cfg.set_heap_size(size);

        self
    }

    /// Set the host print function for the sandbox
    /// # Arguments
    /// * `print_fn` - Function to handle host print calls
    /// # Example
    /// ```
    /// use hyperlight_python::sandbox::SandboxBuilder;
    /// use hyperlight_host::Result;
    ///
    /// fn custom_print(msg: String) -> Result<i32> {
    ///     println!("Sandbox says: {}", msg);
    ///
    ///     Ok(0)
    /// }
    ///
    /// fn main() -> hyperlight_host::Result<()> {
    ///     let sandbox = SandboxBuilder::new()
    ///         .with_host_print_fn(custom_print.into())
    ///         .build()?;
    ///     Ok(())
    /// }
    /// ```
    pub fn with_host_print_fn(mut self, print_fn: HostPrintFn) -> Self {
        self.host_print_fn = Some(print_fn);

        self
    }

    /// Enable debugging for the sandbox created
    /// # Arguments
    /// * `port` - Port to use for debugging
    ///
    /// # Example
    /// ```
    /// use hyperlight_host::sandbox::SandboxBuilder;
    ///
    /// fn main() -> hyperlight_host::Result<()> {
    ///     let sandbox = SandboxBuilder::new()
    ///         .with_debug_enabled(9000)
    ///         .build()?;
    ///     Ok(())
    /// }
    /// ```
    #[cfg(feature = "gdb")]
    pub fn with_debug_enabled(mut self, port: u16) -> Self {
        let dbg_cfg = DebugInfo { port };
        self.cfg.set_guest_debug_info(dbg_cfg);

        self
    }

    /// Use the builder to generate the [`ProtoPySandbox`]
    ///
    /// # Example
    /// ```
    /// use hyperlight_python::sandbox::SandboxBuilder;
    ///
    /// fn main() -> hyperlight_host::Result<()> {
    ///     let sandbox = SandboxBuilder::new()
    ///         .build()?;
    ///     Ok(())
    /// }
    /// ```
    pub fn build(self) -> Result<ProtoPySandbox> {
        if !is_hypervisor_present() {
            return Err(HyperlightError::NoHypervisorFound());
        }
        let guest_binary = GuestBinary::Buffer(super::PYHOST);

        ProtoPySandbox::new(guest_binary, Some(self.cfg), self.host_print_fn)
    }
}
