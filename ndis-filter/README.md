# WireSentinel NDIS LWF Filter Driver

Lightweight NDIS filter for packet classification, redirect, transform, and telemetry.

## Layout

```
ndis-filter/
  classify/     Flow/process classification hooks
  redirect/     Redirect rule application
  transform/    Obfuscation transform pipeline
  telemetry/    Ring buffer event capture
  driver.c      DriverEntry + device creation
  filter.c      NDIS LWF callbacks
  ioctl.c       IOCTL dispatch to usermode controller
  guardian_lwf.inf
  guardian_lwf.vcxproj
```

Build requires Windows Driver Kit (WDK) and Visual Studio. Use `scripts/run-tests.ps1` on Windows CI agents.

Device path: `\\.\WireSentinelNdis`
