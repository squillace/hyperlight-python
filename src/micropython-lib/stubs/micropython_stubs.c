/*
 * MicroPython stubs for bare metal x86 (Hyperlight)
 *
 * These stubs provide the minimal libc-like functions that MicroPython
 * needs to operate in a freestanding environment.
 */

#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>

/* ============================================================================
 * External functions provided by Rust/Hyperlight
 * ============================================================================
 */

// These will be implemented in Rust and linked
extern void hl_print_char(char c);
extern void hl_print_str(const char *s);
extern void hl_abort(void) __attribute__((noreturn));
