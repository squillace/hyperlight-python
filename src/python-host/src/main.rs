//! MicroPython runtime for Hyperlight bare metal
//!
//! This is a standalone binary that embeds the MicroPython interpreter
//! and executes Python code in a Hyperlight virtual machine.

#![no_std]
#![no_main]

extern crate alloc;

/// MicroPython runtime module
mod micropython;

use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::c_char;
use hyperlight_common::flatbuffer_wrappers::function_call::FunctionCall;
use hyperlight_common::flatbuffer_wrappers::function_types::{ParameterValue, ReturnType};
use hyperlight_common::flatbuffer_wrappers::util::get_flatbuffer_result;
use hyperlight_guest::error::Result;
use hyperlight_guest_bin::guest_function;
use hyperlight_guest_bin::host_comm::call_host_function;

use crate::micropython::MicroPython;

/// Static holder for MicroPython runtime (initialized once)
static MP_RUNTIME: spin::Once<MicroPython> = spin::Once::new();

/// This function calls the host to print a message
/// # Arguments
/// * `msg` - The message to print
/// # Returns
/// * `Result<i32>` - The result of the host print call
fn host_print(msg: &str) -> Result<i32> {
    call_host_function::<i32>(
        "HostPrint",
        Some(Vec::from([ParameterValue::String(String::from(msg))])),
        ReturnType::Int,
    )
}

/// Print a single character - called from C stubs
#[unsafe(no_mangle)]
pub extern "C" fn hl_print_char(c: c_char) {
    let ch = c as u8;

    let _ = host_print(core::str::from_utf8(&[ch]).unwrap_or("?"));
}

/// Print a null-terminated string - called from C stubs
#[unsafe(no_mangle)]
pub extern "C" fn hl_print_str(s: *const c_char) {
    let _ = host_print(unsafe {
        core::ffi::CStr::from_ptr(s)
            .to_str()
            .unwrap_or("Invalid UTF-8 string")
    });
}

/// Initialize the MicroPython runtime.
/// This must be called before exec_python.
/// Returns "OK" on success or an error message.
#[guest_function("init_python")]
fn init_python() -> bool {
    if let Some(_) = MP_RUNTIME.get() {
        true
    } else {
        let runtime = MicroPython::init();

        runtime.map(|rt| MP_RUNTIME.call_once(|| rt)).is_ok()
    }
}

/// Execute Python code passed as a string.
/// init_python must be called first.
/// Returns 0 on success or -1.
#[guest_function("exec_python")]
fn exec_python(code: String) -> bool {
    MP_RUNTIME
        .get()
        .map(|mp_runtime| mp_runtime.exec(&code))
        .is_some()
}

#[unsafe(no_mangle)]
pub extern "C" fn hyperlight_main() {}

#[unsafe(no_mangle)]
pub fn guest_dispatch_function(_function_call: FunctionCall) -> Result<Vec<u8>> {
    Ok(get_flatbuffer_result(0))
}
