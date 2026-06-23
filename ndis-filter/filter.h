#pragma once

#include <ndis.h>

#include "../shared/ndis_public.h"
#include "telemetry/telemetry.h"

typedef struct _NDIS_FILTER_CONTEXT {
    NDIS_HANDLE FilterHandle;
    NDIS_HANDLE FilterDriverHandle;
    NDIS_TELEMETRY_RING TelemetryRing;
    NDIS_DRIVER_STATE_V2 DriverState;
    BOOLEAN Attached;
} NDIS_FILTER_CONTEXT, *PNDIS_FILTER_CONTEXT;

DRIVER_INITIALIZE DriverEntry;

FILTER_SET_OPTIONS FilterSetOptions;
FILTER_ATTACH FilterAttach;
FILTER_DETACH FilterDetach;
FILTER_RESTART FilterRestart;
FILTER_PAUSE FilterPause;
FILTER_SEND_NET_BUFFER_LISTS FilterSendNetBufferLists;
FILTER_RECEIVE_NET_BUFFER_LISTS FilterReceiveNetBufferLists;

extern NDIS_FILTER_PARTIAL_CHARACTERISTICS FilterCharacteristics;
