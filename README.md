# WireSentinel-Ndis

NDIS Lightweight Filter stack for WireSentinel — packet classification, per-flow routing, obfuscation transforms, and telemetry.

## Workspace crates

| Crate | Description |
|-------|-------------|
| `packet-engine` | `PacketClassifier` with process/app/route/protocol/flow tracking |
| `route-engine` | `KernelRouteAssignment` and route sync |
| `obfuscation` | Transform, LWO, and cover-traffic engines |
| `telemetry` | `KernelTelemetryV2` ring buffer with batch drain |
| `controller` | `ndis-controller` IOCTL client (`\\.\WireSentinelNdis`) |
| `sdk` | Re-exports and `NdisAgent` helpers |

## Quick start (Linux / CI)

```bash
cargo test --workspace
```

## Windows driver

The kernel filter lives under `ndis-filter/`. Build with Visual Studio + WDK:

```powershell
.\scripts\run-tests.ps1
```

See [docs/architecture.md](docs/architecture.md) for the full Phase 12 design.

## License

Apache-2.0
