#include "classify.h"

NTSTATUS
NdisFilterClassifyPacket(
    _In_reads_bytes_(Length) const UCHAR* Frame,
    _In_ ULONG Length,
    _In_ ULONG ProcessId,
    _Out_ PNDIS_ROUTE_ASSIGNMENT_V2 Assignment
    )
{
    UNREFERENCED_PARAMETER(Frame);
    UNREFERENCED_PARAMETER(ProcessId);

    if (Assignment == NULL) {
        return STATUS_INVALID_PARAMETER;
    }

    RtlZeroMemory(Assignment, sizeof(*Assignment));
    Assignment->Version = NDIS_ROUTE_ASSIGNMENT_VERSION;
    Assignment->RouteKind = NdisRouteDirect;
    Assignment->Protocol = (Length > 9) ? Frame[9] : 0;
    return STATUS_SUCCESS;
}
