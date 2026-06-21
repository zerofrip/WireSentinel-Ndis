use ndis_controller::{CoverTrafficMode, NdisCoverTrafficProfileV2};

#[derive(Debug, Clone)]
pub struct CoverBurst {
    pub payload: Vec<u8>,
    pub interval_ms: u32,
}

pub struct KernelCoverTrafficEngine {
    profile: NdisCoverTrafficProfileV2,
    sequence: u64,
}

impl KernelCoverTrafficEngine {
    pub fn from_mode(mode: CoverTrafficMode) -> Self {
        Self {
            profile: NdisCoverTrafficProfileV2::from_mode(mode),
            sequence: 0,
        }
    }

    pub fn enabled(&self) -> bool {
        self.profile.enabled != 0
    }

    pub fn mode(&self) -> CoverTrafficMode {
        match self.profile.mode {
            0 => CoverTrafficMode::Off,
            1 => CoverTrafficMode::Low,
            2 => CoverTrafficMode::Medium,
            3 => CoverTrafficMode::High,
            _ => CoverTrafficMode::Off,
        }
    }

    pub fn profile(&self) -> &NdisCoverTrafficProfileV2 {
        &self.profile
    }

    pub fn next_burst(&mut self) -> Option<CoverBurst> {
        if !self.enabled() {
            return None;
        }
        self.sequence = self.sequence.wrapping_add(1);
        let payload_span = self
            .profile
            .max_payload_bytes
            .saturating_sub(self.profile.min_payload_bytes)
            + 1;
        let interval_span = self
            .profile
            .max_interval_ms
            .saturating_sub(self.profile.min_interval_ms)
            + 1;
        let size = self.profile.min_payload_bytes + ((self.sequence as u32) % payload_span);
        let interval = self.profile.min_interval_ms + ((self.sequence as u32) % interval_span);
        let mut payload = vec![0u8; size as usize];
        for (idx, byte) in payload.iter_mut().enumerate() {
            *byte = ((self.sequence as u8).wrapping_add(idx as u8)) ^ 0xA5;
        }
        Some(CoverBurst {
            payload,
            interval_ms: interval,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cover_traffic_disabled_when_off() {
        let mut engine = KernelCoverTrafficEngine::from_mode(CoverTrafficMode::Off);
        assert!(!engine.enabled());
        assert!(engine.next_burst().is_none());
    }

    #[test]
    fn cover_traffic_generates_payload() {
        let mut engine = KernelCoverTrafficEngine::from_mode(CoverTrafficMode::Low);
        let burst = engine.next_burst().expect("burst");
        assert!(!burst.payload.is_empty());
        assert!(burst.interval_ms >= engine.profile().min_interval_ms);
    }
}
