#pragma once

#include <stdint.h>

#define NDIS_TELEMETRY_VERSION 2
#define NDIS_TELEMETRY_RING_CAPACITY 256

#pragma pack(push, 1)

typedef enum _NDIS_TELEMETRY_EVENT_KIND {
    NdisTelemetryClassify = 0,
    NdisTelemetryRedirect = 1,
    NdisTelemetryTransform = 2,
    NdisTelemetryCoverTraffic = 3,
    NdisTelemetryError = 4,
} NDIS_TELEMETRY_EVENT_KIND;

typedef struct _NDIS_TELEMETRY_EVENT_V2 {
    UINT32 Version;
    UINT32 Kind;
    UINT64 Timestamp100ns;
    UINT8 FlowId[16];
    UINT32 ProcessId;
    UINT32 Protocol;
    UINT64 Bytes;
    INT32 ResultCode;
    UINT8 Reserved[4];
} NDIS_TELEMETRY_EVENT_V2, *PNDIS_TELEMETRY_EVENT_V2;

typedef struct _NDIS_TELEMETRY_SUMMARY_V2 {
    UINT32 Version;
    UINT64 ClassifyCount;
    UINT64 RedirectCount;
    UINT64 TransformCount;
    UINT64 CoverTrafficCount;
    UINT64 ErrorCount;
    UINT64 DroppedCount;
    UINT64 AvgClassifyLatency100ns;
    UINT64 MaxClassifyLatency100ns;
    UINT32 RingHead;
    UINT32 RingTail;
    UINT32 RingCapacity;
    UINT32 PendingEvents;
} NDIS_TELEMETRY_SUMMARY_V2, *PNDIS_TELEMETRY_SUMMARY_V2;

typedef struct _NDIS_TELEMETRY_BATCH_V2 {
    UINT32 Version;
    UINT32 EventCount;
    NDIS_TELEMETRY_EVENT_V2 Events[1];
} NDIS_TELEMETRY_BATCH_V2, *PNDIS_TELEMETRY_BATCH_V2;

#pragma pack(pop)
