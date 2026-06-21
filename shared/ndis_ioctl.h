#pragma once

/* Shared IOCTL definitions — WireSentinelNdis.sys + usermode controller */

#ifdef _KERNEL_MODE
#include <ntddk.h>
#include <wdm.h>
#else
#include <windows.h>
#endif

#include "structures/route_assignment_v2.h"
#include "structures/telemetry_v2.h"
#include "structures/transform_profile.h"
#include "structures/cover_traffic.h"
#include "structures/redirect.h"

#define NDIS_DEVICE_NAME L"\\Device\\WireSentinelNdis"
#define NDIS_SYMBOLIC_NAME L"\\DosDevices\\WireSentinelNdis"
#define NDIS_USER_DEVICE_PATH L"\\\\.\\WireSentinelNdis"

#define NDIS_IOCTL_DEVICE_TYPE 0x8020u
#define NDIS_PROTOCOL_VERSION 2u

#define NDIS_DRIVER_STATE_VERSION 2u

#pragma pack(push, 1)

typedef struct _NDIS_DRIVER_STATE_V2 {
    UINT32 Version;
    UINT32 VersionMajor;
    UINT32 VersionMinor;
    UINT32 VersionPatch;
    UINT32 BuildStamp;
    UINT32 LifecycleState;
    UINT32 FilterAttached;
    UINT32 ActiveRouteCount;
    UINT32 ActiveRedirectCount;
    INT32 LastError;
    UINT8 Reserved[12];
} NDIS_DRIVER_STATE_V2, *PNDIS_DRIVER_STATE_V2;

#pragma pack(pop)

#define IOCTL_NDIS_GET_DRIVER_STATE \
    CTL_CODE(NDIS_IOCTL_DEVICE_TYPE, 0x900, METHOD_BUFFERED, FILE_READ_DATA)

#define IOCTL_NDIS_SET_ROUTE \
    CTL_CODE(NDIS_IOCTL_DEVICE_TYPE, 0x901, METHOD_BUFFERED, FILE_WRITE_DATA)

#define IOCTL_NDIS_CLEAR_ROUTE \
    CTL_CODE(NDIS_IOCTL_DEVICE_TYPE, 0x902, METHOD_BUFFERED, FILE_WRITE_DATA)

#define IOCTL_NDIS_SYNC_ROUTES \
    CTL_CODE(NDIS_IOCTL_DEVICE_TYPE, 0x903, METHOD_BUFFERED, FILE_WRITE_DATA)

#define IOCTL_NDIS_SET_TRANSFORM_PROFILE \
    CTL_CODE(NDIS_IOCTL_DEVICE_TYPE, 0x904, METHOD_BUFFERED, FILE_WRITE_DATA)

#define IOCTL_NDIS_SET_COVER_TRAFFIC \
    CTL_CODE(NDIS_IOCTL_DEVICE_TYPE, 0x905, METHOD_BUFFERED, FILE_WRITE_DATA)

#define IOCTL_NDIS_SYNC_REDIRECT \
    CTL_CODE(NDIS_IOCTL_DEVICE_TYPE, 0x906, METHOD_BUFFERED, FILE_WRITE_DATA)

#define IOCTL_NDIS_GET_TELEMETRY_SUMMARY \
    CTL_CODE(NDIS_IOCTL_DEVICE_TYPE, 0x907, METHOD_BUFFERED, FILE_READ_DATA)

#define IOCTL_NDIS_DRAIN_TELEMETRY \
    CTL_CODE(NDIS_IOCTL_DEVICE_TYPE, 0x908, METHOD_BUFFERED, FILE_READ_DATA)

#define IOCTL_NDIS_CLASSIFY_PACKET \
    CTL_CODE(NDIS_IOCTL_DEVICE_TYPE, 0x909, METHOD_BUFFERED, FILE_WRITE_DATA)

#define NDIS_MAX_IOCTL_BUFFER 65536u
