# rummage - Collect compile time and runtime environment information for debug logging

[![Latest Version]][crates.io] [![Documentation]][docs.rs] ![License]

`rummage` provides a simple and opinionated way to collect a reasonably
comprehensive set of compile time and runtime environment information, and emit
it in structured `tracing` events.

## Example usage

```rust
fn main() {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Collect the default info set, plus the three named environment variables
    let info = rummage::info!().with_envvars(["RUST_LOG", "HOME", "PWD"]);
    
    // Emit the gathered info as a set of structured log messages
    info.log_debug();
}
```

<details>
<summary>
Which emits something like the following (formatted with `jq` for readability):
</summary>

```json
{
  "timestamp": "2023-03-15T23:14:00.634339Z",
  "level": "DEBUG",
  "fields": {
    "message": "Crate information:",
    "git_commit_hash": "3acb2d157c80c720a306e1ea11c5d026167cfeda",
    "git_repo_dirty": true,
    "crate_name": "rummage_demo",
    "crate_version": "0.1.0",
    "bin_name": "rummage-demo"
  },
  "target": "rummage"
}
{
  "timestamp": "2023-03-15T23:14:00.634413Z",
  "level": "DEBUG",
  "fields": {
    "message": "Cargo target information:",
    "profile": "debug",
    "host": "x86_64-unknown-linux-gnu",
    "target": "x86_64-unknown-linux-gnu",
    "family": "unix",
    "os": "linux",
    "arch": "x86_64",
    "pointer_width": "64",
    "endian": "little",
    "features": "fxsr,sse,sse2"
  },
  "target": "rummage"
}
{
  "timestamp": "2023-03-15T23:14:00.634437Z",
  "level": "DEBUG",
  "fields": {
    "message": "Rustc information:",
    "rustc_semver": "1.68.0",
    "commit_hash": "2c8cc343237b8f7d5a3c3703e3a87f2eb2c54a74",
    "commit_date": "2023-03-06",
    "llvm_version": "15.0"
  },
  "target": "rummage"
}
{
  "timestamp": "2023-03-15T23:14:00.634451Z",
  "level": "DEBUG",
  "fields": {
    "message": "System information:",
    "hostname": "DESKTOP-0E64KFA",
    "os": "Linux 5.10.102.1-microsoft-standard-WSL2",
    "linux_distro": "Ubuntu 22.04.1 LTS",
    "cpu_vendor": "AuthenticAMD",
    "cpu_brand_string": "AMD Ryzen 9 3950X 16-Core Processor"
  },
  "target": "rummage"
}
{
  "timestamp": "2023-03-15T23:14:00.634469Z",
  "level": "DEBUG",
  "fields": {
    "message": "Command line args:",
    "args": "[\"target/debug/rummage-demo\", \"--foo=10\", \"--bar\", \"apple\"]"
  },
  "target": "rummage"
}
{
  "timestamp": "2023-03-15T23:14:00.634484Z",
  "level": "DEBUG",
  "fields": {
    "message": "Environment variables:",
    "args": "{\"RUST_LOG\": Some(\"debug\"), \"HOME\": Some(\"/home/joe\"), \"PWD\": Some(\"/home/joe/dev/rummage-demo\")}"
  },
  "target": "rummage"
}
```
</details>


[Latest Version]: https://img.shields.io/crates/v/rummage.svg
[Documentation]: https://docs.rs/rummage/badge.svg
[License]: https://img.shields.io/crates/l/rummage.svg
[crates.io]: https://crates.io/crates/rummage
[docs.rs]: https://docs.rs/rummage/latest/rummage