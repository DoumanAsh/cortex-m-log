# cortex-m-log

![Rust](https://github.com/DoumanAsh/cortex-m-log/workflows/Rust/badge.svg?branch=master)
[![Crates.io](https://img.shields.io/crates/v/cortex-m-log.svg)](https://crates.io/crates/cortex-m-log)
[![Documentation](https://docs.rs/cortex-m-log/badge.svg)](https://docs.rs/crate/cortex-m-log/)
[![dependency status](https://deps.rs/repo/github/DoumanAsh/cortex-m-log/status.svg)](https://deps.rs/repo/github/DoumanAsh/cortex-m-log)

Logging facilities for Cortex-M processors

## Available features

- `log-integration` - Enables [log](https://github.com/rust-lang-nursery/log) integration
- `semihosting` - Enables facilities for [cortex-m-semihosting](https://github.com/japaric/cortex-m-semihosting).
- `itm` - Enables ITM destination for logging (not available on Cortex-M0 microcontrollers)

## Maintenance note

Please note that at the current moment I'm no longer working with cortex chips and unlikely to work on improving this crate further.
If you wish to take over, please let me know and I'll consider giving full rights to access this crate.
