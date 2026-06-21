#include "telemetry.h"

VOID
NdisTelemetryRingInit(
    _Out_ PNDIS_TELEMETRY_RING Ring
    )
{
    RtlZeroMemory(Ring, sizeof(*Ring));
}

NTSTATUS
NdisTelemetryRingPush(
    _Inout_ PNDIS_TELEMETRY_RING Ring,
    _In_ const NDIS_TELEMETRY_EVENT_V2* Event
    )
{
    if (Ring == NULL || Event == NULL) {
        return STATUS_INVALID_PARAMETER;
    }

    if (Ring->Count >= NDIS_TELEMETRY_RING_CAPACITY) {
        return STATUS_BUFFER_OVERFLOW;
    }

    Ring->Events[Ring->Tail] = *Event;
    Ring->Tail = (Ring->Tail + 1) % NDIS_TELEMETRY_RING_CAPACITY;
    Ring->Count++;
    return STATUS_SUCCESS;
}

ULONG
NdisTelemetryRingDrain(
    _Inout_ PNDIS_TELEMETRY_RING Ring,
    _Out_writes_(MaxEvents) NDIS_TELEMETRY_EVENT_V2* OutEvents,
    _In_ ULONG MaxEvents
    )
{
    ULONG drained = 0;

    if (Ring == NULL || OutEvents == NULL || MaxEvents == 0) {
        return 0;
    }

    while (Ring->Count > 0 && drained < MaxEvents) {
        OutEvents[drained++] = Ring->Events[Ring->Head];
        Ring->Head = (Ring->Head + 1) % NDIS_TELEMETRY_RING_CAPACITY;
        Ring->Count--;
    }

    return drained;
}
