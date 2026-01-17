use std::path::{Path, PathBuf};
use std::{env, fs};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("host_resource.rs");
    let _ = fs::remove_file(&dest_path);

    bundle_host();
}

fn resolve_python_runtime_path() -> PathBuf {
    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let manifest_dir = PathBuf::from(manifest_dir);

    let tar_path = manifest_dir.join("vendor.tar");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_dir = PathBuf::from(out_dir);
    let vendor_dir = out_dir.join("vendor");

    if vendor_dir.exists() {
        fs::remove_dir_all(&vendor_dir).unwrap();
    }

    println!("cargo::rerun-if-changed={}", tar_path.display());

    // If the vendor.tar file exists, extract it to the OUT_DIR/vendor directory
    // and return the python_runtime directory inside it.
    // This is useful for vendoring the python-host crate in a release build, since crates.io
    // does not allow vendoring folders with Cargo.toml files (i.e., other crates).
    // The vendor.tar file is expected to be in the same directory as this build script.
    if tar_path.exists() {
        let mut tar = tar::Archive::new(fs::File::open(&tar_path).unwrap());
        tar.unpack(&vendor_dir).unwrap();

        let python_runtime_dir = vendor_dir.join("python-host");

        println!(
            "cargo::warning=using vendor python-host from {}",
            tar_path.display()
        );
        return python_runtime_dir;
    }

    let crates_dir = manifest_dir.parent().unwrap();

    println!("{}\n{}", crates_dir.display(), vendor_dir.display());
    #[cfg(unix)]
    std::os::unix::fs::symlink(crates_dir, &vendor_dir).unwrap();

    #[cfg(not(unix))]
    junction::create(crates_dir, &vendor_dir).unwrap();

    let python_runtime_dir = crates_dir.join("python-host");
    if python_runtime_dir.exists() {
        return python_runtime_dir;
    }

    panic!(
        r#"
        The python_runtime directory not found in the expected locations.
        If you are using hyperlight-python from a registry release, please file an issue: https://github.com/hyperlight-org/hyperlight-python/issues
        "#
    );
}

fn build_python_runtime() -> PathBuf {
    let profile = env::var_os("PROFILE").unwrap();
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let target_dir = Path::new(&out_dir).join("target").join("python-host");

    let in_repo_dir = resolve_python_runtime_path();

    if !in_repo_dir.exists() {
        panic!(
            "missing python-host in-tree dependency at {:?}",
            in_repo_dir
        );
    }

    println!("cargo::rerun-if-changed={}", in_repo_dir.display());
    // the PROFILE env var unfortunately only gives us 1 bit of "dev or release"
    let cargo_profile = if profile == "debug" { "dev" } else { "release" };

    let stubs_inc = in_repo_dir.join("stubs").join("include");
    let cflags = format!("-I{} -D__wasi__=1", stubs_inc.display());

    let mut cargo_cmd = cargo_hyperlight::cargo().unwrap();
    let cmd = cargo_cmd
        .arg("build")
        .arg("--profile")
        .arg(cargo_profile)
        .arg("-v")
        .arg("--target-dir")
        .arg(&target_dir)
        .current_dir(&in_repo_dir)
        .env_clear_cargo()
        .env("HYPERLIGHT_CFLAGS", cflags);

    cmd.status()
        .unwrap_or_else(|e| panic!("Could not run cargo build python runtime: {e:?}\n{cmd:?}"));

    let resource = target_dir
        .join("x86_64-hyperlight-none")
        .join(profile)
        .join("python-host");

    if let Ok(path) = resource.canonicalize() {
        if std::env::var("CARGO_FEATURE_GDB").is_ok() {
            println!(
                "cargo:warning=Python runtime guest binary at: {}",
                path.display()
            );
        }
        path
    } else {
        panic!(
            "could not find python runtime after building it (expected {:?})",
            resource
        )
    }
}

fn bundle_host() {
    let python_runtime_resource = build_python_runtime();

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("host_resource.rs");
    let contents =
        format!("pub (super) static PYHOST: &[u8] = include_bytes!({python_runtime_resource:?});");

    fs::write(dest_path, contents).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
