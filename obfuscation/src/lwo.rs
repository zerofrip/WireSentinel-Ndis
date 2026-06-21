use ndis_controller::ObfuscationPreset;

pub struct KernelLwoEngine {
    preset: ObfuscationPreset,
    magic: u32,
}

impl KernelLwoEngine {
    pub fn new() -> Self {
        Self {
            preset: ObfuscationPreset::Lwo,
            magic: 0x4C574F00,
        }
    }

    pub fn preset(&self) -> ObfuscationPreset {
        self.preset
    }

    pub fn obfuscate(&self, payload: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(payload.len() + 8);
        out.extend_from_slice(&self.magic.to_le_bytes());
        out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        for (idx, byte) in payload.iter().enumerate() {
            out.push(byte ^ ((idx as u8).wrapping_mul(13)));
        }
        out
    }

    pub fn deobfuscate(&self, payload: &[u8]) -> Option<Vec<u8>> {
        if payload.len() < 8 {
            return None;
        }
        let magic = u32::from_le_bytes(payload[0..4].try_into().ok()?);
        if magic != self.magic {
            return None;
        }
        let len = u32::from_le_bytes(payload[4..8].try_into().ok()?) as usize;
        let body = payload.get(8..8 + len)?;
        Some(
            body.iter()
                .enumerate()
                .map(|(idx, byte)| byte ^ ((idx as u8).wrapping_mul(13)))
                .collect(),
        )
    }
}

impl Default for KernelLwoEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lwo_roundtrip() {
        let engine = KernelLwoEngine::new();
        let input = b"wireguard-handshake";
        let obfuscated = engine.obfuscate(input);
        let restored = engine.deobfuscate(&obfuscated).expect("roundtrip");
        assert_eq!(restored, input);
    }
}
