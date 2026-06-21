#pragma once

#include <stdint.h>

#define NDIS_ROUTE_ASSIGNMENT_VERSION 2
#define NDIS_MAX_SYNC_ROUTES 512

typedef enum _NDIS_ROUTE_KIND {
    NdisRouteDirect = 0,
    NdisRouteVpn = 1,
    NdisRouteTailnet = 2,
    NdisRouteTor = 3,
    NdisRouteAnonymous = 4,
    NdisRouteBlocked = 5,
    NdisRouteProxy = 6,
    NdisRouteChain = 7,
} NDIS_ROUTE_KIND;

#pragma pack(push, 1)

typedef struct _NDIS_ROUTE_ASSIGNMENT_V2 {
    UINT32 Version;
    UINT8 FlowId[16];
    UINT8 AppId[16];
    UINT32 RouteKind;
    UINT64 ProfileId;
    UINT64 InterfaceLuid;
    UINT16 SocksPort;
    UINT16 Protocol;
    UINT8 Reserved[4];
} NDIS_ROUTE_ASSIGNMENT_V2, *PNDIS_ROUTE_ASSIGNMENT_V2;

typedef struct _NDIS_SYNC_ROUTES_V2 {
    UINT32 Version;
    UINT32 RouteCount;
    NDIS_ROUTE_ASSIGNMENT_V2 Routes[1];
} NDIS_SYNC_ROUTES_V2, *PNDIS_SYNC_ROUTES_V2;

#pragma pack(pop)
