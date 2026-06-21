#include "ioctl.h"
#include "filter.h"
#include "telemetry/telemetry.h"

static NDIS_DRIVER_STATE_V2 g_NdisDriverState;

static BOOLEAN
NdisCheckStructureVersion(
    _In_reads_bytes_(BufferLength) const VOID* Buffer,
    _In_ ULONG BufferLength,
    _In_ UINT32 ExpectedVersion
    )
{
    if (Buffer == NULL || BufferLength < sizeof(UINT32)) {
        return FALSE;
    }

    return (*(const UINT32*)Buffer == ExpectedVersion);
}

NTSTATUS
NdisValidateIoctlBuffer(
    _In_ ULONG IoControlCode,
    _In_ ULONG InputBufferLength,
    _In_ ULONG OutputBufferLength,
    _In_reads_bytes_opt_(InputBufferLength) const VOID* InputBuffer,
    _Out_ PULONG RequiredOutput
    )
{
    if (RequiredOutput == NULL) {
        return STATUS_INVALID_PARAMETER;
    }

    *RequiredOutput = 0;

    if (InputBufferLength > NDIS_MAX_IOCTL_BUFFER ||
        OutputBufferLength > NDIS_MAX_IOCTL_BUFFER) {
        return STATUS_INVALID_BUFFER_SIZE;
    }

    switch (IoControlCode) {
    case IOCTL_NDIS_GET_DRIVER_STATE:
        *RequiredOutput = (ULONG)sizeof(g_NdisDriverState);
        break;

    case IOCTL_NDIS_SET_ROUTE:
        if (InputBuffer == NULL ||
            InputBufferLength < sizeof(NDIS_ROUTE_ASSIGNMENT_V2) ||
            !NdisCheckStructureVersion(
                InputBuffer,
                InputBufferLength,
                NDIS_ROUTE_ASSIGNMENT_VERSION)) {
            return STATUS_INVALID_PARAMETER;
        }
        break;

    case IOCTL_NDIS_CLEAR_ROUTE:
        if (InputBuffer == NULL || InputBufferLength < 16) {
            return STATUS_INVALID_PARAMETER;
        }
        break;

    case IOCTL_NDIS_SYNC_ROUTES: {
        const NDIS_SYNC_ROUTES_V2* sync;
        ULONG needed;

        if (InputBuffer == NULL || InputBufferLength < sizeof(UINT32) * 2) {
            return STATUS_INVALID_PARAMETER;
        }

        sync = (const NDIS_SYNC_ROUTES_V2*)InputBuffer;
        if (sync->Version != NDIS_ROUTE_ASSIGNMENT_VERSION ||
            sync->RouteCount > NDIS_MAX_SYNC_ROUTES) {
            return STATUS_INVALID_PARAMETER;
        }

        needed = (ULONG)(sizeof(UINT32) * 2 +
            sync->RouteCount * sizeof(NDIS_ROUTE_ASSIGNMENT_V2));
        if (InputBufferLength < needed) {
            return STATUS_INVALID_PARAMETER;
        }
        break;
    }

    case IOCTL_NDIS_SET_TRANSFORM_PROFILE:
        if (InputBuffer == NULL ||
            InputBufferLength < sizeof(NDIS_TRANSFORM_PROFILE_V2) ||
            !NdisCheckStructureVersion(
                InputBuffer,
                InputBufferLength,
                NDIS_TRANSFORM_PROFILE_VERSION)) {
            return STATUS_INVALID_PARAMETER;
        }
        break;

    case IOCTL_NDIS_SET_COVER_TRAFFIC:
        if (InputBuffer == NULL ||
            InputBufferLength < sizeof(NDIS_COVER_TRAFFIC_PROFILE_V2) ||
            !NdisCheckStructureVersion(
                InputBuffer,
                InputBufferLength,
                NDIS_COVER_TRAFFIC_VERSION)) {
            return STATUS_INVALID_PARAMETER;
        }
        break;

    case IOCTL_NDIS_SYNC_REDIRECT: {
        const NDIS_SYNC_REDIRECT_V2* sync;
        ULONG needed;

        if (InputBuffer == NULL || InputBufferLength < sizeof(UINT32) * 2) {
            return STATUS_INVALID_PARAMETER;
        }

        sync = (const NDIS_SYNC_REDIRECT_V2*)InputBuffer;
        if (sync->Version != NDIS_REDIRECT_RULE_VERSION ||
            sync->RuleCount > NDIS_MAX_REDIRECT_RULES) {
            return STATUS_INVALID_PARAMETER;
        }

        needed = (ULONG)(sizeof(UINT32) * 2 +
            sync->RuleCount * sizeof(NDIS_REDIRECT_RULE_V2));
        if (InputBufferLength < needed) {
            return STATUS_INVALID_PARAMETER;
        }
        break;
    }

    case IOCTL_NDIS_GET_TELEMETRY_SUMMARY:
        *RequiredOutput = (ULONG)sizeof(NDIS_TELEMETRY_SUMMARY_V2);
        break;

    case IOCTL_NDIS_DRAIN_TELEMETRY:
        if (OutputBufferLength < sizeof(UINT32) * 2) {
            return STATUS_BUFFER_TOO_SMALL;
        }
        *RequiredOutput = (ULONG)(sizeof(UINT32) * 2);
        break;

    case IOCTL_NDIS_CLASSIFY_PACKET:
        if (InputBuffer == NULL || InputBufferLength < sizeof(UINT32) * 2) {
            return STATUS_INVALID_PARAMETER;
        }
        break;

    default:
        return STATUS_INVALID_DEVICE_REQUEST;
    }

    if (*RequiredOutput > OutputBufferLength) {
        return STATUS_BUFFER_TOO_SMALL;
    }

    return STATUS_SUCCESS;
}

NTSTATUS
NdisDeviceCreate(
    _In_ PDRIVER_OBJECT DriverObject
    )
{
    UNICODE_STRING deviceName;
    UNICODE_STRING symbolicName;
    PDEVICE_OBJECT deviceObject = NULL;
    NTSTATUS status;

    RtlInitUnicodeString(&deviceName, NDIS_DEVICE_NAME);
    RtlInitUnicodeString(&symbolicName, NDIS_SYMBOLIC_NAME);

    status = IoCreateDevice(
        DriverObject,
        0,
        &deviceName,
        FILE_DEVICE_UNKNOWN,
        FILE_DEVICE_SECURE_OPEN,
        FALSE,
        &deviceObject);

    if (!NT_SUCCESS(status)) {
        return status;
    }

    status = IoCreateSymbolicLink(&symbolicName, &deviceName);
    if (!NT_SUCCESS(status)) {
        IoDeleteDevice(deviceObject);
        return status;
    }

    DriverObject->MajorFunction[IRP_MJ_CREATE] = NdisIoctlDispatch;
    DriverObject->MajorFunction[IRP_MJ_CLOSE] = NdisIoctlDispatch;
    DriverObject->MajorFunction[IRP_MJ_DEVICE_CONTROL] = NdisIoctlDispatch;

    RtlZeroMemory(&g_NdisDriverState, sizeof(g_NdisDriverState));
    g_NdisDriverState.Version = NDIS_DRIVER_STATE_VERSION;
    g_NdisDriverState.VersionMajor = 0;
    g_NdisDriverState.VersionMinor = 1;
    g_NdisDriverState.VersionPatch = 0;
    g_NdisDriverState.LifecycleState = 1;

    return STATUS_SUCCESS;
}

VOID
NdisDeviceDelete(
    _In_ PDRIVER_OBJECT DriverObject
    )
{
    UNICODE_STRING symbolicName;

    RtlInitUnicodeString(&symbolicName, NDIS_SYMBOLIC_NAME);
    IoDeleteSymbolicLink(&symbolicName);

    if (DriverObject->DeviceObject != NULL) {
        IoDeleteDevice(DriverObject->DeviceObject);
    }
}

NTSTATUS
NdisIoctlDispatch(
    _In_ PDEVICE_OBJECT DeviceObject,
    _Inout_ PIRP Irp
    )
{
    PIO_STACK_LOCATION stack;
    NTSTATUS status = STATUS_SUCCESS;
    ULONG bytesReturned = 0;
    ULONG requiredOutput = 0;
    PVOID systemBuffer;

    UNREFERENCED_PARAMETER(DeviceObject);

    stack = IoGetCurrentIrpStackLocation(Irp);
    systemBuffer = Irp->AssociatedIrp.SystemBuffer;

    status = NdisValidateIoctlBuffer(
        stack->Parameters.DeviceIoControl.IoControlCode,
        stack->Parameters.DeviceIoControl.InputBufferLength,
        stack->Parameters.DeviceIoControl.OutputBufferLength,
        systemBuffer,
        &requiredOutput);
    if (!NT_SUCCESS(status)) {
        goto Complete;
    }

    switch (stack->Parameters.DeviceIoControl.IoControlCode) {
    case IOCTL_NDIS_GET_DRIVER_STATE:
        RtlCopyMemory(systemBuffer, &g_NdisDriverState, sizeof(g_NdisDriverState));
        bytesReturned = sizeof(g_NdisDriverState);
        break;

    default:
        status = STATUS_INVALID_DEVICE_REQUEST;
        break;
    }

Complete:
    Irp->IoStatus.Status = status;
    Irp->IoStatus.Information = bytesReturned;
    IoCompleteRequest(Irp, IO_NO_INCREMENT);
    return status;
}
