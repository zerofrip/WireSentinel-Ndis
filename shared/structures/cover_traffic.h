#pragma once

#include "../ndis_types.h"

#define NDIS_COVER_TRAFFIC_VERSION 2

typedef enum _NDIS_COVER_TRAFFIC_MODE {
    NdisCoverTrafficOff = 0,
    NdisCoverTrafficLow = 1,
    NdisCoverTrafficMedium = 2,
    NdisCoverTrafficHigh = 3,
} NDIS_COVER_TRAFFIC_MODE;

#pragma pack(push, 1)

typedef struct _NDIS_COVER_TRAFFIC_PROFILE_V2 {
    UINT32 Version;
    UINT32 Mode;
    UINT32 MinIntervalMs;
    UINT32 MaxIntervalMs;
    UINT32 MinPayloadBytes;
    UINT32 MaxPayloadBytes;
    UINT32 BurstCount;
    UINT8 Enabled;
    UINT8 Reserved[3];
} NDIS_COVER_TRAFFIC_PROFILE_V2, *PNDIS_COVER_TRAFFIC_PROFILE_V2;

#pragma pack(pop)
