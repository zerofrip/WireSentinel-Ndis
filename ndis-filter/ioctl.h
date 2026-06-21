#pragma once

#include "../shared/ndis_public.h"

NTSTATUS
NdisDeviceCreate(
    _In_ PDRIVER_OBJECT DriverObject
    );

VOID
NdisDeviceDelete(
    _In_ PDRIVER_OBJECT DriverObject
    );

NTSTATUS
NdisValidateIoctlBuffer(
    _In_ ULONG IoControlCode,
    _In_ ULONG InputBufferLength,
    _In_ ULONG OutputBufferLength,
    _In_reads_bytes_opt_(InputBufferLength) const VOID* InputBuffer,
    _Out_ PULONG RequiredOutput
    );

NTSTATUS
NdisIoctlDispatch(
    _In_ PDEVICE_OBJECT DeviceObject,
    _Inout_ PIRP Irp
    );
