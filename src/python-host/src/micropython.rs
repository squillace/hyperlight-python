//! Safe Rust wrapper for the embedded MicroPython runtime.
//!
//! This module provides a safe interface to initialize, run Python code,
//! and deinitialize the MicroPython interpreter.

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::ffi::c_void;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;
use hyperlight_guest::error::{HyperlightGuestError, Result};
use spin::Mutex;

/// Size of the MicroPython garbage collector heap (32 KB)
pub const DEFAULT_HEAP_SIZE: usize = 32 * 1024;

/// Static heap for MicroPython's garbage collector.
/// This needs to be static so it lives for the duration of the program.
static MP_HEAP: spin::Once<Mutex<Vec<u8>>> = spin::Once::new();

/// Global variable that enforces only one MicroPython instance to ensure
/// the stack is only used by one
static MP_INITIALIZED: Mutex<AtomicBool> = Mutex::new(AtomicBool::new(false));

/// Represents an initialized MicroPython runtime.
///
/// This struct ensures proper initialization and cleanup of the MicroPython
/// interpreter through RAII. When dropped, it will deinitialize the runtime.
pub struct MicroPython {
    _private: (), // Prevent direct construction
}

impl MicroPython {
    /// Initialize a new MicroPython runtime.
    ///
    /// # Safety
    /// This function is safe to call, but only one MicroPython instance
    /// should exist at a time. The runtime uses global state internally.
    ///
    /// # Returns
    /// A `MicroPython` instance that will deinitialize the runtime when dropped.
    pub fn init() -> Result<Self> {
        // Fail early if runtime already initialized
        if let Some(guard) = MP_INITIALIZED.try_lock() {
            if guard.load(Ordering::Acquire) {
                return Err(HyperlightGuestError::new(
                    hyperlight_common::flatbuffer_wrappers::guest_error::ErrorCode::GuestError,
                    "MicroPython already initialized".to_string(),
                ));
            }
        }

        MP_HEAP.call_once(|| Mutex::new(Vec::with_capacity(DEFAULT_HEAP_SIZE)));

        if let Some(heap) = MP_HEAP.get() {
            let mut guard = heap.try_lock().ok_or(HyperlightGuestError::new(
                hyperlight_common::flatbuffer_wrappers::guest_error::ErrorCode::GuestError,
                "Cannot lock heap for python runtime".to_string(),
            ))?;

            let heap_ptr = guard.as_mut_ptr() as *mut c_void;
            let heap_size = guard.capacity();

            // Use the address of a local variable as stack top estimate
            let mut stack_marker: usize = 0;
            let stack_ptr = &mut stack_marker as *mut usize as *mut c_void;

            unsafe {
                micropython_lib::mp_embed_init(heap_ptr, heap_size, stack_ptr);
            }

            // Mark as initialized
            if let Some(guard) = MP_INITIALIZED.try_lock() {
                guard.store(true, Ordering::Release);
            }

            Ok(MicroPython { _private: () })
        } else {
            Err(HyperlightGuestError::new(
                hyperlight_common::flatbuffer_wrappers::guest_error::ErrorCode::GuestError,
                "Heap not initialized".to_string(),
            ))
        }
    }

    /// Execute a Python source string.
    ///
    /// # Arguments
    /// * `code` - A string slice containing Python source code to execute.
    ///
    /// # Note
    /// Any output from the Python code (via `print()`) will be sent through
    /// the Hyperlight host call mechanism.
    pub fn exec(&self, code: &str) {
        // We need to ensure the string is null-terminated for C
        // Since we're in no_std, we'll use a stack buffer
        let mut buf = String::with_capacity(code.len() + 1);

        buf.push_str(code);
        buf.push('\0');

        unsafe {
            micropython_lib::mp_embed_exec_str(buf.as_ptr() as *const core::ffi::c_char);
        }
    }

    /// Execute a Python source string (static version for longer code).
    ///
    /// # Arguments
    /// * `code` - A null-terminated C string (must end with \0).
    ///
    /// # Safety
    /// The `code` string must be null-terminated.
    #[allow(dead_code)]
    pub fn exec_cstr(&self, code: *const core::ffi::c_char) {
        unsafe {
            micropython_lib::mp_embed_exec_str(code);
        }
    }
}

impl Drop for MicroPython {
    fn drop(&mut self) {
        // Mark runtime as uninitialized
        if let Some(guard) = MP_INITIALIZED.try_lock() {
            guard.store(false, Ordering::Release);
        }

        // Deinitialize the MicroPython runtime
        unsafe {
            micropython_lib::mp_embed_deinit();
        }
    }
}
