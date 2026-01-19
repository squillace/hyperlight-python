use hyperlight_python::sandbox::SandboxBuilder;

fn main() -> hyperlight_host::Result<()> {
    let builder = SandboxBuilder::new();
    #[cfg(feature = "gdb")]
    let builder = builder.with_debug_enabled(8080);

    let proto_sbox = builder.build()?;

    let code = r"print('Hello from Python sandbox!')".to_string();

    let sandbox = proto_sbox.load_runtime()?;
    let mut sandbox = sandbox.get_loaded_sandbox()?;

    let success = sandbox.run_script(code)?;
    assert!(success);
    Ok(())
}
