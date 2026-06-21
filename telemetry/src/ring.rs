use ndis_controller::NdisTelemetryEventV2;

#[derive(Debug, Clone, Default)]
pub struct DrainBatch {
    pub events: Vec<NdisTelemetryEventV2>,
}

pub struct TelemetryRingBuffer {
    capacity: u32,
    buffer: Vec<Option<NdisTelemetryEventV2>>,
    head: u32,
    tail: u32,
    len: u32,
}

impl TelemetryRingBuffer {
    pub fn new(capacity: u32) -> Self {
        let cap = capacity.max(1);
        Self {
            capacity: cap,
            buffer: vec![None; cap as usize],
            head: 0,
            tail: 0,
            len: 0,
        }
    }

    pub fn len(&self) -> u32 {
        self.len
    }

    pub fn head(&self) -> u32 {
        self.head
    }

    pub fn tail(&self) -> u32 {
        self.tail
    }

    pub fn push(&mut self, event: NdisTelemetryEventV2) -> Result<(), ()> {
        if self.len >= self.capacity {
            return Err(());
        }
        self.buffer[self.tail as usize] = Some(event);
        self.tail = (self.tail + 1) % self.capacity;
        self.len += 1;
        Ok(())
    }

    pub fn drain(&mut self, max_events: usize) -> DrainBatch {
        let take = max_events.min(self.len as usize);
        let mut events = Vec::with_capacity(take);
        for _ in 0..take {
            if self.len == 0 {
                break;
            }
            if let Some(event) = self.buffer[self.head as usize].take() {
                events.push(event);
            }
            self.head = (self.head + 1) % self.capacity;
            self.len -= 1;
        }
        DrainBatch { events }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndis_controller::{NDIS_TELEMETRY_VERSION, TelemetryEventKind};

    fn sample_event(kind: TelemetryEventKind) -> NdisTelemetryEventV2 {
        NdisTelemetryEventV2 {
            version: NDIS_TELEMETRY_VERSION,
            kind: kind as u32,
            timestamp_100ns: 1,
            flow_id: [0; 16],
            process_id: 1,
            protocol: 17,
            bytes: 64,
            result_code: 0,
            reserved: [0; 4],
        }
    }

    #[test]
    fn ring_buffer_drops_when_full() {
        let mut ring = TelemetryRingBuffer::new(2);
        assert!(ring.push(sample_event(TelemetryEventKind::Classify)).is_ok());
        assert!(ring.push(sample_event(TelemetryEventKind::Redirect)).is_ok());
        assert!(ring.push(sample_event(TelemetryEventKind::Error)).is_err());
        let batch = ring.drain(10);
        assert_eq!(batch.events.len(), 2);
    }
}
