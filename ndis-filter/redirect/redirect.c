#include "redirect.h"

NTSTATUS
NdisFilterApplyRedirect(
    _In_ const NDIS_REDIRECT_RULE_V2* Rule,
    _Inout_ PULONG Action
    )
{
    if (Rule == NULL || Action == NULL) {
        return STATUS_INVALID_PARAMETER;
    }

    *Action = Rule->Action;
    return STATUS_SUCCESS;
}
