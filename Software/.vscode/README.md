# VS Code configuration

Extensions: `rust-lang.rust-analyzer`, `probe-rs.probe-rs-debugger`, and
optionally `marus25.cortex-debug` for the OpenOCD configuration.

* **Debug (probe-rs)**: builds, flashes and debugs over any probe-rs probe;
  defmt logs show in the debug console.
* **Debug (OpenOCD)**: uses `../openocd.cfg` (edit it for your probe).

For peripheral register views, add the STM32F407 SVD file here and point
`svdFile` at it.
