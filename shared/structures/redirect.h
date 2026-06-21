#pragma once

#include <stdint.h>

#define NDIS_REDIRECT_RULE_VERSION 2
#define NDIS_MAX_REDIRECT_RULES 256

typedef enum _NDIS_REDIRECT_ACTION {
    NdisRedirectPass = 0,
    NdisRedirectInject = 1,
    NdisRedirectDrop = 2,
    NdisRedirectLoopback = 3,
} NDIS_REDIRECT_ACTION;

#pragma pack(push, 1)

typedef struct _NDIS_REDIRECT_RULE_V2 {
    UINT32 Version;
    UINT8 FlowId[16];
    UINT32 Action;
    UINT64 TargetInterfaceLuid;
    UINT16 TargetPort;
    UINT16 Protocol;
    UINT8 Reserved[4];
} NDIS_REDIRECT_RULE_V2, *PNDIS_REDIRECT_RULE_V2;

typedef struct _NDIS_SYNC_REDIRECT_V2 {
    UINT32 Version;
    UINT32 RuleCount;
    NDIS_REDIRECT_RULE_V2 Rules[1];
} NDIS_SYNC_REDIRECT_V2, *PNDIS_SYNC_REDIRECT_V2;

#pragma pack(pop)
