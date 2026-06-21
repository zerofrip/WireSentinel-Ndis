use crate::ring::DrainBatch;
use ndis_controller::{
    uuid_to_bytes, NdisTelemetryEventV2, NdisTelemetrySummaryV2, TelemetryEventKind,
    NDIS_TELEMETRY_RING_CAPACITY, NDIS_TELEMETRY_VERSION,
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TelemetryRecord {
    pub kind: TelemetryEventKind,
    pub flow_id: Uuid,
    pub process_id: u32,
    pub protocol: u32,
    pub bytes: u64,
    pub result_code: i32,
    pub timestamp_100ns: u64,
}

pub struct KernelTelemetryV2 {
    ring: super::ring::TelemetryRingBuffer,
    summary: NdisTelemetrySummaryV2,
    clock_100ns: u64,
}

impl Default for KernelTelemetryV2 {
    fn default() -> Self {
        Self::new()
    }
}

impl KernelTelemetryV2 {
    pub fn new() -> Self {
        Self {
            ring: super::ring::TelemetryRingBuffer::new(NDIS_TELEMETRY_RING_CAPACITY),
            summary: NdisTelemetrySummaryV2 {
                version: NDIS_TELEMETRY_VERSION,
                ring_capacity: NDIS_TELEMETRY_RING_CAPACITY,
                ..Default::default()
            },
            clock_100ns: 0,
        }
    }

    pub fn record(&mut self, record: TelemetryRecord) {
        self.clock_100ns = self.clock_100ns.saturating_add(10_000);
        let event = NdisTelemetryEventV2 {
            version: NDIS_TELEMETRY_VERSION,
            kind: record.kind as u32,
            timestamp_100ns: record.timestamp_100ns.max(self.clock_100ns),
            flow_id: uuid_to_bytes(record.flow_id),
            process_id: record.process_id,
            protocol: record.protocol,
            bytes: record.bytes,
            result_code: record.result_code,
            reserved: [0; 4],
        };
        match record.kind {
            TelemetryEventKind::Classify => self.summary.classify_count += 1,
            TelemetryEventKind::Redirect => self.summary.redirect_count += 1,
            TelemetryEventKind::Transform => self.summary.transform_count += 1,
            TelemetryEventKind::CoverTraffic => self.summary.cover_traffic_count += 1,
            TelemetryEventKind::Error => self.summary.error_count += 1,
        }
        if record.result_code < 0 {
            self.summary.error_count += 1;
        }
        if self.ring.push(event).is_err() {
            self.summary.dropped_count += 1;
        }
        self.summary.pending_events = self.ring.len() as u32;
        self.summary.ring_head = self.ring.head();
        self.summary.ring_tail = self.ring.tail();
    }

    pub fn summary(&self) -> &NdisTelemetrySummaryV2 {
        &self.summary
    }

    pub fn drain(&mut self, max_events: usize) -> DrainBatch {
        let batch = self.ring.drain(max_events);
        self.summary.pending_events = self.ring.len() as u32;
        self.summary.ring_head = self.ring.head();
        self.summary.ring_tail = self.ring.tail();
        batch
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_and_drains_events() {
        let mut telemetry = KernelTelemetryV2::new();
        telemetry.record(TelemetryRecord {
            kind: TelemetryEventKind::Classify,
            flow_id: Uuid::new_v4(),
            process_id: 100,
            protocol: 6,
            bytes: 512,
            result_code: 0,
            timestamp_100ns: 0,
        });
        let classify_count = unsafe {
            std::ptr::read_unaligned(std::ptr::addr_of!(telemetry.summary().classify_count))
        };
        assert_eq!(classify_count, 1);
        let batch = telemetry.drain(16);
        assert_eq!(batch.events.len(), 1);
        let pending = unsafe {
            std::ptr::read_unaligned(std::ptr::addr_of!(telemetry.summary().pending_events))
        };
        assert_eq!(pending, 0);
    }
}
