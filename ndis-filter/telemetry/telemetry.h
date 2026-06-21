#pragma once

#include "../shared/ndis_public.h"

typedef struct _NDIS_TELEMETRY_RING {
    NDIS_TELEMETRY_EVENT_V2 Events[NDIS_TELEMETRY_RING_CAPACITY];
    ULONG Head;
    ULONG Tail;
    ULONG Count;
} NDIS_TELEMETRY_RING, *PNDIS_TELEMETRY_RING;

VOID
NdisTelemetryRingInit(
    _Out_ PNDIS_TELEMETRY_RING Ring
    );

NTSTATUS
NdisTelemetryRingPush(
    _Inout_ PNDIS_TELEMETRY_RING Ring,
    _In_ const NDIS_TELEMETRY_EVENT_V2* Event
    );

ULONG
NdisTelemetryRingDrain(
    _Inout_ PNDIS_TELEMETRY_RING Ring,
    _Out_writes_(MaxEvents) NDIS_TELEMETRY_EVENT_V2* OutEvents,
    _In_ ULONG MaxEvents
    );
