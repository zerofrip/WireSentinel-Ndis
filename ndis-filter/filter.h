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
