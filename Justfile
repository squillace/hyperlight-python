set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]
default-target:= "debug"
set-env-command := if os() == "windows" { "$env:" } else { "export " }

# in windows we need to replace the backslashes with forward slashes
# otherwise clang will misinterpret the paths
PWD := replace(justfile_dir(), "\\", "/")

# On Windows, use Ninja generator for CMake to avoid aws-lc-sys build issues with Visual Studio generator
export CMAKE_GENERATOR := if os() == "windows" { "Ninja" } else { "" }

ensure-tools:
    cargo install cargo-hyperlight --locked --version 0.1.3

build target=default-target features="": (build-rust target features)

build-rust target=default-target features="":
    @echo "Building hyperlight-python for target: {{target}} with features: {{features}}"
    cargo build --profile={{ if target == "debug" { "dev" } else { target } }} --features "{{features}}"

