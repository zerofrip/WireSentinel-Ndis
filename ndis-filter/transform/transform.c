#include "transform.h"

NTSTATUS
NdisFilterApplyTransform(
    _In_ const NDIS_TRANSFORM_PROFILE_V2* Profile,
    _Inout_updates_bytes_(Length) UCHAR* Buffer,
    _In_ ULONG Length
    )
{
    UNREFERENCED_PARAMETER(Buffer);

    if (Profile == NULL) {
        return STATUS_INVALID_PARAMETER;
    }

    if (Profile->Preset == NdisObfuscationDisabled || Length == 0) {
        return STATUS_SUCCESS;
    }

    return STATUS_SUCCESS;
}
