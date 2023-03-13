# rummage - Collect compiletime and runtime environment collector

[![Latest Version]][crates.io] [![Documentation]][docs.rs] ![License]

`rummage` provides a simple and opinionated way to collect a reasonably
comprehensive set of compile time and runtime environment information, and emit
it in structured `tracing` events.

## Example usage

```rust
fn main() {
    tracing_subscriber::fmt::fmt().init();

    // Collect the default info set, plus the three named environment variables
    let info = rummage::info!().with_envvars(["RUST_LOG", "HOME", "PWD"]);
    info.log_debug();
}
```

Which emits:
```
~/dev/rummage-demo$ RUST_LOG=debug cargo run -- --foo=10 --bar apple
   Compiling rummage v0.1.0
   Compiling rummage-demo v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.64s
     Running `target/debug/rummage-demo --foo=10 --bar apple`
2023-03-13T23:06:50.183994Z DEBUG rummage: Crate information: git_commit_hash="3acb2d1" crate_name="rummage_demo" crate_version="0.1.0" bin_name="rummage-demo"
2023-03-13T23:06:50.184046Z DEBUG rummage: Cargo target information: profile="debug" host="x86_64-unknown-linux-gnu" target="x86_64-unknown-linux-gnu" family="unix" os="linux" arch="x86_64" pointer_width="64" endian="little" features="fxsr,sse,sse2"
2023-03-13T23:06:50.184092Z DEBUG rummage: Rustc information: rustc_semver="1.68.0" commit_hash="2c8cc343237b8f7d5a3c3703e3a87f2eb2c54a74" commit_date="2023-03-06" llvm_version="15.0"
2023-03-13T23:06:50.184126Z DEBUG rummage: System information hostname="DESKTOP-0E64KFA" os="Linux 5.10.102.1-microsoft-standard-WSL2" linux_distro="Ubuntu 22.04.1 LTS" cpu_vendor="AuthenticAMD" cpu_brand_string="AMD Ryzen 9 3950X 16-Core Processor"
2023-03-13T23:06:50.184167Z DEBUG rummage: Command line args: args="[\"target/debug/rummage-demo\", \"--foo=10\", \"--bar\", \"apple\"]"
2023-03-13T23:06:50.184200Z DEBUG rummage: Collected environment variables args="{\"HOME\": Some(\"/home/joe\"), \"PWD\": Some(\"/home/joe/dev/rummage-demo\"), \"RUST_LOG\": Some(\"debug\")}"
```


[Latest Version]: https://img.shields.io/crates/v/rummage.svg
[Documentation]: https://docs.rs/rummage/badge.svg
[License]: https://img.shields.io/crates/l/rummage.svg
[crates.io]: https://crates.io/crates/rummage
[docs.rs]: https://docs.rs/rummage/latest/rummage