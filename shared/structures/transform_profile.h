#pragma once

#include "../ndis_types.h"

#define NDIS_TRANSFORM_PROFILE_VERSION 2
#define NDIS_MAX_TRANSFORM_MODULES 8

typedef enum _NDIS_OBFUSCATION_PRESET {
    NdisObfuscationDisabled = 0,
    NdisObfuscationBasic = 1,
    NdisObfuscationBalanced = 2,
    NdisObfuscationAggressive = 3,
    NdisObfuscationLwo = 4,
} NDIS_OBFUSCATION_PRESET;

typedef enum _NDIS_TRANSFORM_MODULE_KIND {
    NdisTransformPadding = 0,
    NdisTransformJitter = 1,
    NdisTransformFragment = 2,
    NdisTransformCamouflage = 3,
    NdisTransformLwo = 4,
} NDIS_TRANSFORM_MODULE_KIND;

#pragma pack(push, 1)

typedef struct _NDIS_TRANSFORM_MODULE_V2 {
    UINT32 Kind;
    UINT32 Parameter0;
    UINT32 Parameter1;
    UINT32 Reserved;
} NDIS_TRANSFORM_MODULE_V2, *PNDIS_TRANSFORM_MODULE_V2;

typedef struct _NDIS_TRANSFORM_PROFILE_V2 {
    UINT32 Version;
    UINT32 Preset;
    UINT32 ModuleCount;
    NDIS_TRANSFORM_MODULE_V2 Modules[NDIS_MAX_TRANSFORM_MODULES];
} NDIS_TRANSFORM_PROFILE_V2, *PNDIS_TRANSFORM_PROFILE_V2;

#pragma pack(pop)
