#pragma once

#ifdef _KERNEL_MODE
#include <ntddk.h>
#else
#include <stdint.h>

typedef uint8_t UINT8;
typedef uint16_t UINT16;
typedef uint32_t UINT32;
typedef uint64_t UINT64;
typedef int32_t INT32;

#endif
