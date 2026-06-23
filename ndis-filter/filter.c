#include "filter.h"
#include "ioctl.h"
#include "classify/classify.h"
#include "redirect/redirect.h"
#include "transform/transform.h"
#include "telemetry/telemetry.h"

static NDIS_HANDLE g_FilterDriverHandle = NULL;

_Use_decl_annotations_
NDIS_STATUS
FilterSetOptions(
    NDIS_HANDLE NdisFilterDriverHandle,
    NDIS_HANDLE FilterDriverContext
    )
{
    UNREFERENCED_PARAMETER(NdisFilterDriverHandle);
    UNREFERENCED_PARAMETER(FilterDriverContext);
    return NDIS_STATUS_SUCCESS;
}

_Use_decl_annotations_
NDIS_STATUS
FilterAttach(
    NDIS_HANDLE NdisFilterHandle,
    NDIS_HANDLE FilterDriverContext,
    PNDIS_FILTER_ATTACH_PARAMETERS AttachParameters
    )
{
    PNDIS_FILTER_CONTEXT context;
    NDIS_STATUS status;
    NDIS_FILTER_ATTRIBUTES attributes;

    UNREFERENCED_PARAMETER(FilterDriverContext);

    context = (PNDIS_FILTER_CONTEXT)NdisAllocateMemoryWithTagPriority(
        NdisFilterHandle,
        sizeof(NDIS_FILTER_CONTEXT),
        'sdiN',
        NormalPoolPriority);

    if (context == NULL) {
        return NDIS_STATUS_RESOURCES;
    }

    RtlZeroMemory(context, sizeof(*context));
    context->FilterHandle = NdisFilterHandle;
    context->Attached = TRUE;
    NdisTelemetryRingInit(&context->TelemetryRing);

    attributes.Header.Type = NDIS_OBJECT_TYPE_FILTER_ATTRIBUTES;
    attributes.Header.Revision = NDIS_FILTER_ATTRIBUTES_REVISION_1;
    attributes.Header.Size = NDIS_SIZEOF_FILTER_ATTRIBUTES_REVISION_1;
    attributes.Flags = 0;

    status = NdisFSetAttributes(NdisFilterHandle, context, &attributes);
    if (status != NDIS_STATUS_SUCCESS) {
        NdisFreeMemory(context, 0, 0);
        return status;
    }

    return NDIS_STATUS_SUCCESS;
}

_Use_decl_annotations_
VOID
FilterDetach(
    NDIS_HANDLE FilterModuleContext
    )
{
    PNDIS_FILTER_CONTEXT context = (PNDIS_FILTER_CONTEXT)FilterModuleContext;

    if (context != NULL) {
        context->Attached = FALSE;
        NdisFreeMemory(context, 0, 0);
    }
}

_Use_decl_annotations_
NDIS_STATUS
FilterRestart(
    NDIS_HANDLE FilterModuleContext,
    PNDIS_FILTER_RESTART_PARAMETERS RestartParameters
    )
{
    UNREFERENCED_PARAMETER(FilterModuleContext);
    UNREFERENCED_PARAMETER(RestartParameters);
    return NDIS_STATUS_SUCCESS;
}

_Use_decl_annotations_
NDIS_STATUS
FilterPause(
    NDIS_HANDLE FilterModuleContext,
    PNDIS_FILTER_PAUSE_PARAMETERS PauseParameters
    )
{
    UNREFERENCED_PARAMETER(FilterModuleContext);
    UNREFERENCED_PARAMETER(PauseParameters);
    return NDIS_STATUS_SUCCESS;
}

_Use_decl_annotations_
VOID
FilterSendNetBufferLists(
    NDIS_HANDLE FilterModuleContext,
    PNET_BUFFER_LIST NetBufferLists,
    NDIS_PORT_NUMBER PortNumber,
    ULONG SendFlags
    )
{
    PNDIS_FILTER_CONTEXT context = (PNDIS_FILTER_CONTEXT)FilterModuleContext;

    if (context != NULL && context->Attached) {
        NdisFSendNetBufferLists(
            context->FilterHandle,
            NetBufferLists,
            PortNumber,
            SendFlags);
    }
}

_Use_decl_annotations_
VOID
FilterReceiveNetBufferLists(
    NDIS_HANDLE FilterModuleContext,
    PNET_BUFFER_LIST NetBufferLists,
    NDIS_PORT_NUMBER PortNumber,
    ULONG NumberOfNetBufferLists,
    ULONG ReceiveFlags
    )
{
    PNDIS_FILTER_CONTEXT context = (PNDIS_FILTER_CONTEXT)FilterModuleContext;

    if (context != NULL && context->Attached) {
        NdisFIndicateReceiveNetBufferLists(
            context->FilterHandle,
            NetBufferLists,
            PortNumber,
            NumberOfNetBufferLists,
            ReceiveFlags);
    }
}

_Use_decl_annotations_
NTSTATUS
DriverEntry(
    PDRIVER_OBJECT DriverObject,
    PUNICODE_STRING RegistryPath
    )
{
    NDIS_STATUS status;
    NDIS_FILTER_DRIVER_CHARACTERISTICS fdc;

    UNREFERENCED_PARAMETER(RegistryPath);

    RtlZeroMemory(&fdc, sizeof(fdc));
    fdc.Header.Type = NDIS_OBJECT_TYPE_FILTER_DRIVER_CHARACTERISTICS;
    fdc.Header.Revision = NDIS_FILTER_CHARACTERISTICS_REVISION_3;
    fdc.Header.Size = NDIS_SIZEOF_FILTER_DRIVER_CHARACTERISTICS_REVISION_3;
    fdc.MajorNdisVersion = 6;
    fdc.MinorNdisVersion = 80;
    fdc.MajorDriverVersion = 0;
    fdc.MinorDriverVersion = 1;
    fdc.Flags = 0;

    fdc.AttachHandler = FilterAttach;
    fdc.DetachHandler = FilterDetach;
    fdc.RestartHandler = FilterRestart;
    fdc.PauseHandler = FilterPause;
    fdc.SendNetBufferListsHandler = FilterSendNetBufferLists;
    fdc.ReturnNetBufferListsHandler = NULL;
    fdc.ReceiveNetBufferListsHandler = FilterReceiveNetBufferLists;
    fdc.SetOptionsHandler = FilterSetOptions;

    RtlInitUnicodeString(&fdc.FriendlyName, L"WireSentinel NDIS Filter");
    RtlInitUnicodeString(&fdc.UniqueName, L"WireSentinelNdisFilter");
    RtlInitUnicodeString(&fdc.ServiceName, L"WireSentinelNdis");

    status = NdisFRegisterFilterDriver(
        DriverObject,
        (NDIS_HANDLE)DriverObject,
        &fdc,
        &g_FilterDriverHandle);

    if (status != NDIS_STATUS_SUCCESS) {
        return STATUS_UNSUCCESSFUL;
    }

    return NdisDeviceCreate(DriverObject);
}
