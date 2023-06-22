fn main() {
    // Re-export variables that are only available at build.rs-time, but not at compile time.
    for var in &[
        "PROFILE",
        "TARGET",
        "CARGO_CFG_TARGET_FAMILY",
        "CARGO_CFG_TARGET_OS",
        "CARGO_CFG_TARGET_ARCH",
        "CARGO_CFG_TARGET_POINTER_WIDTH",
        "CARGO_CFG_TARGET_ENDIAN",
        "CARGO_CFG_TARGET_FEATURE",
        "HOST",
    ] {
        println!(
            "cargo:rustc-env=RUMMAGE_{}={}",
            var,
            std::env::var(var).unwrap_or_else(|_| "unknown".into())
        );
    }

    let rustc_version =
        rustc_version::version_meta().expect("Failed to gather rustc version informatonn");
    println!(
        "cargo:rustc-env=RUMMAGE_RUSTC_VERSION_MAJOR={}",
        rustc_version.semver.major
    );
    println!(
        "cargo:rustc-env=RUMMAGE_RUSTC_VERSION_MINOR={}",
        rustc_version.semver.minor
    );
    println!(
        "cargo:rustc-env=RUMMAGE_RUSTC_VERSION_PATCH={}",
        rustc_version.semver.patch
    );
    println!(
        "cargo:rustc-env=RUMMAGE_RUSTC_VERSION_PRE={}",
        rustc_version.semver.pre
    );
    println!(
        "cargo:rustc-env=RUMMAGE_RUSTC_VERSION_BUILD={}",
        rustc_version.semver.build
    );
    println!(
        "cargo:rustc-env=RUMMAGE_RUSTC_VERSION_COMMIT_HASH={}",
        rustc_version.commit_hash.unwrap_or_default()
    );
    println!(
        "cargo:rustc-env=RUMMAGE_RUSTC_VERSION_COMMIT_DATE={}",
        rustc_version.commit_date.unwrap_or_default()
    );
    println!(
        "cargo:rustc-env=RUMMAGE_RUSTC_VERSION_LLVM_VERSION={}",
        rustc_version
            .llvm_version
            .map(|l| format!("{}.{}", l.major, l.minor))
            .unwrap_or_default()
    );
}
