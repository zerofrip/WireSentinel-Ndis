#pragma once

#include "../shared/ndis_public.h"

NTSTATUS
NdisFilterApplyTransform(
    _In_ const NDIS_TRANSFORM_PROFILE_V2* Profile,
    _Inout_updates_bytes_(Length) UCHAR* Buffer,
    _In_ ULONG Length
    );
