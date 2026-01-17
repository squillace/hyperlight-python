use hyperlight_python::sandbox::SandboxBuilder;

fn main() -> hyperlight_host::Result<()> {
    let proto_sbox = SandboxBuilder::new().build()?;

    let code = r#"
class Example:
    def __init__(self, msg):
        self.msg = msg

    def __str__(self):
        return "Message from class {}: {}".format(__class__.__qualname__, self.msg)

example = Example('Hello from Python sandbox!')
print(example)
"#
    .to_string();

    let sandbox = proto_sbox.load_runtime()?;
    let mut sandbox = sandbox.get_loaded_sandbox()?;

    let success = sandbox.run_script(code)?;
    assert!(success);
    Ok(())
}
