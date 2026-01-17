/*
 * MicroPython configuration for Hyperlight bare metal x86 VM
 * 
 * This configuration disables OS-dependent features and configures
 * MicroPython to run in a freestanding environment.
 */

#ifndef MPCONFIGPORT_H
#define MPCONFIGPORT_H

#include <stdint.h>

// ============================================================================
// Type definitions for x86_64 bare metal
// ============================================================================

// mp_int_t and mp_uint_t are defined by MicroPython based on architecture
// We only need to define mp_off_t for file offsets
typedef long mp_off_t;
typedef long ssize_t;

// Use minimum ROM level - no fancy features
#define MICROPY_CONFIG_ROM_LEVEL                (MICROPY_CONFIG_ROM_LEVEL_MINIMUM)

// Use MicroPython's internal GC (required for bare metal)
#define MICROPY_ENABLE_COMPILER                 (1)
#define MICROPY_ENABLE_GC                       (1)

// Disable all optional modules that require OS support
#define MICROPY_PY_GC                           (1)
#define MICROPY_PY_SYS                          (0)
#define MICROPY_PY_ARRAY                        (1)

#define MICROPY_MPHALPORT_H                     "port/mphalport.h"

// Provide a simple port configuration
#define MICROPY_HW_BOARD_NAME                   "Hyperlight-x86"
#define MICROPY_HW_MCU_NAME                     "x86_64"

// ============================================================================
// Bare metal: provide our own alloca (only if not already defined)
// ============================================================================

#ifndef alloca
#define alloca(sz) __builtin_alloca(sz)
#endif

// ============================================================================
// NLR (Non-Local Return) configuration for exception handling
// ============================================================================

// Use setjmp-based NLR for x86_64 (portable)
#define MICROPY_NLR_SETJMP                      (1)

// ============================================================================
// Debug configuration
// ============================================================================

#define MICROPY_DEBUG_PRINTERS                  (0)
#define MICROPY_DEBUG_VERBOSE                   (0)

#ifndef SEEK_CUR
#define SEEK_CUR (1)
#endif

#ifndef SEEK_SET
#define SEEK_SET (0)
#endif

#endif // MPCONFIGPORT_H
