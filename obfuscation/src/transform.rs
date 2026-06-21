use ndis_controller::{
    NdisTransformModuleV2, NdisTransformProfileV2, ObfuscationPreset, TransformModuleKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformStep {
    pub kind: TransformModuleKind,
    pub output_len: usize,
}

pub struct KernelTransformEngine {
    profile: NdisTransformProfileV2,
}

impl KernelTransformEngine {
    pub fn from_preset(preset: ObfuscationPreset) -> Self {
        Self {
            profile: NdisTransformProfileV2::from_preset(preset),
        }
    }

    pub fn preset(&self) -> ObfuscationPreset {
        match self.profile.preset {
            0 => ObfuscationPreset::Disabled,
            1 => ObfuscationPreset::Basic,
            2 => ObfuscationPreset::Balanced,
            3 => ObfuscationPreset::Aggressive,
            4 => ObfuscationPreset::Lwo,
            _ => ObfuscationPreset::Disabled,
        }
    }

    pub fn module_count(&self) -> u32 {
        self.profile.module_count
    }

    pub fn modules(&self) -> &[NdisTransformModuleV2] {
        &self.profile.modules[..self.profile.module_count as usize]
    }

    pub fn profile(&self) -> &NdisTransformProfileV2 {
        &self.profile
    }

    pub fn transform(&self, payload: &[u8]) -> Vec<u8> {
        let mut current = payload.to_vec();
        for module in self.modules() {
            current = apply_module(*module, &current);
        }
        current
    }

    pub fn plan(&self, payload_len: usize) -> Vec<TransformStep> {
        self.modules()
            .iter()
            .map(|module| {
                let kind = module_kind(module.kind);
                let output_len = estimate_output_len(kind, payload_len);
                TransformStep { kind, output_len }
            })
            .collect()
    }
}

fn apply_module(module: NdisTransformModuleV2, input: &[u8]) -> Vec<u8> {
    match module_kind(module.kind) {
        TransformModuleKind::Padding => {
            let pad = (module.parameter0.max(1) % 16) as usize;
            let mut out = input.to_vec();
            out.extend(std::iter::repeat(0u8).take(pad));
            out
        }
        TransformModuleKind::Jitter => {
            let mut out = input.to_vec();
            if !out.is_empty() {
                out[0] ^= 0x01;
            }
            out
        }
        TransformModuleKind::Fragment => {
            let mid = input.len() / 2;
            let mut out = vec![mid as u8];
            out.extend_from_slice(input);
            out
        }
        TransformModuleKind::Camouflage => {
            let mut out = vec![0x16, 0x03, 0x01];
            out.extend_from_slice(input);
            out
        }
        TransformModuleKind::Lwo => input.to_vec(),
    }
}

fn estimate_output_len(kind: TransformModuleKind, input_len: usize) -> usize {
    match kind {
        TransformModuleKind::Padding => input_len + 8,
        TransformModuleKind::Jitter => input_len,
        TransformModuleKind::Fragment => input_len + 1,
        TransformModuleKind::Camouflage => input_len + 3,
        TransformModuleKind::Lwo => input_len,
    }
}

fn module_kind(value: u32) -> TransformModuleKind {
    match value {
        0 => TransformModuleKind::Padding,
        1 => TransformModuleKind::Jitter,
        2 => TransformModuleKind::Fragment,
        3 => TransformModuleKind::Camouflage,
        4 => TransformModuleKind::Lwo,
        _ => TransformModuleKind::Padding,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presets_match_module_lists() {
        for preset in [
            ObfuscationPreset::Disabled,
            ObfuscationPreset::Basic,
            ObfuscationPreset::Balanced,
            ObfuscationPreset::Aggressive,
            ObfuscationPreset::Lwo,
        ] {
            let engine = KernelTransformEngine::from_preset(preset);
            assert_eq!(
                engine.modules().len(),
                ndis_controller::preset_modules(preset).len()
            );
        }
    }

    #[test]
    fn aggressive_transform_grows_payload() {
        let engine = KernelTransformEngine::from_preset(ObfuscationPreset::Aggressive);
        let out = engine.transform(b"hello");
        assert!(out.len() > 5);
    }
}
