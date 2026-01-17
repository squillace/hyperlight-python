//! Build script for python-host
//!
//! The MicroPython compilation is handled by the micropython-lib crate.
//! This build script is kept minimal.

fn main() {
    // Nothing to do - micropython-lib handles the C compilation
    println!("cargo:rerun-if-changed=build.rs");
}
