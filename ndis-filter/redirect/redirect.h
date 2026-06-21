#pragma once

#include "../shared/ndis_public.h"

NTSTATUS
NdisFilterApplyRedirect(
    _In_ const NDIS_REDIRECT_RULE_V2* Rule,
    _Inout_ PULONG Action
    );
