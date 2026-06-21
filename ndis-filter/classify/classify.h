#pragma once

#include "../shared/ndis_public.h"

NTSTATUS
NdisFilterClassifyPacket(
    _In_reads_bytes_(Length) const UCHAR* Frame,
    _In_ ULONG Length,
    _In_ ULONG ProcessId,
    _Out_ PNDIS_ROUTE_ASSIGNMENT_V2 Assignment
    );
