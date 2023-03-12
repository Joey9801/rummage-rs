//! # Rummage
//!
//! Collect various buildtime and runtime properties, and provides an easy way to dump those
//! properties into [`tracing`] event.

use std::collections::HashMap;

/// Re-export so dependent project does not have to manually depend on git-version crate
pub use git_version::git_version;
use tracing::Level;

#[derive(Debug)]
pub struct CrateInfo {
    pub git_commit_hash: String,
    pub crate_name: String,
    pub crate_version: String,
    pub bin_name: String,
}

impl CrateInfo {
    fn log_debug(&self) {
        tracing::event!(
            Level::DEBUG,
            git_commit_hash = self.git_commit_hash,
            crate_name = self.crate_name,
            crate_version = self.crate_version,
            bin_name = self.bin_name,
            "Crate information:"
        );
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! _crate_info {
    () => {{
        ::rummage::CrateInfo {
            git_commit_hash: ::rummage::git_version!(fallback = "").to_string(),
            crate_name: env!("CARGO_CRATE_NAME").to_string(),
            crate_version: env!("CARGO_PKG_VERSION").to_string(),
            bin_name: env!("CARGO_BIN_NAME").to_string(),
        }
    }};
}

#[derive(Debug)]
pub struct CargoTarget {
    pub profile: String,
    pub host: String,
    pub target: String,
    pub family: String,
    pub os: String,
    pub arch: String,
    pub pointer_width: String,
    pub endian: String,
    pub features: String,
}

impl CargoTarget {
    /// This method only works when called from within the rummage crate, as it relies on cargo
    /// environment variables rummage sets in its own build.rs
    #[doc(hidden)]
    pub fn gather() -> Self {
        Self {
            profile: env!("RUMMAGE_PROFILE").to_string(),
            host: env!("RUMMAGE_HOST").to_string(),
            target: env!("RUMMAGE_TARGET").to_string(),
            family: env!("RUMMAGE_CARGO_CFG_TARGET_FAMILY").to_string(),
            os: env!("RUMMAGE_CARGO_CFG_TARGET_OS").to_string(),
            arch: env!("RUMMAGE_CARGO_CFG_TARGET_ARCH").to_string(),
            pointer_width: env!("RUMMAGE_CARGO_CFG_TARGET_POINTER_WIDTH").to_string(),
            endian: env!("RUMMAGE_CARGO_CFG_TARGET_ENDIAN").to_string(),
            features: env!("RUMMAGE_CARGO_CFG_TARGET_FEATURE").to_string(),
        }
    }

    pub fn log_debug(&self) {
        tracing::event!(
            Level::DEBUG,
            profile = self.profile,
            host = self.host,
            target = self.target,
            family = self.family,
            os = self.os,
            arch = self.arch,
            pointer_width = self.pointer_width,
            endian = self.endian,
            features = self.features,
            "Cargo target information:"
        )
    }
}

#[derive(Debug)]
pub struct RustcVersion {
    pub rustc_semver: String,
    pub commit_hash: String,
    pub commit_date: String,
    pub llvm_version: String,
}

impl RustcVersion {
    /// This method only works when called from within the rummage crate, as it relies on cargo
    /// environment variables rummage sets in its own build.rs
    #[doc(hidden)]
    pub fn gather() -> Self {
        let major = env!("RUMMAGE_RUSTC_VERSION_MAJOR");
        let minor = env!("RUMMAGE_RUSTC_VERSION_MINOR");
        let patch = env!("RUMMAGE_RUSTC_VERSION_PATCH");
        let pre = env!("RUMMAGE_RUSTC_VERSION_PRE");
        let build = env!("RUMMAGE_RUSTC_VERSION_BUILD");

        let mut rustc_semver = format!("{major}.{minor}.{patch}");
        if !pre.is_empty() {
            rustc_semver.push('-');
            rustc_semver.push_str(pre);
        }

        if !build.is_empty() {
            rustc_semver.push('+');
            rustc_semver.push_str(build);
        }

        Self {
            rustc_semver,
            commit_hash: env!("RUMMAGE_RUSTC_VERSION_COMMIT_HASH").to_string(),
            commit_date: env!("RUMMAGE_RUSTC_VERSION_COMMIT_DATE").to_string(),
            llvm_version: env!("RUMMAGE_RUSTC_VERSION_LLVM_VERSION").to_string(),
        }
    }

    pub fn log_debug(&self) {
        tracing::event!(
            Level::DEBUG,
            rustc_semver = self.rustc_semver,
            commit_hash = self.commit_hash,
            commit_date = self.commit_date,
            llvm_version = self.llvm_version,
            "Rustc information:"
        );
    }
}

#[derive(Debug)]
pub struct CompileInfo {
    pub target: CargoTarget,
    pub rustc: RustcVersion,
}

impl CompileInfo {
    #[doc(hidden)]
    pub fn gather() -> Self {
        Self {
            target: CargoTarget::gather(),
            rustc: RustcVersion::gather(),
        }
    }

    pub fn log_debug(&self) {
        self.target.log_debug();
        self.rustc.log_debug();
    }
}

#[derive(Debug)]
pub struct CpuInfo {
    pub vendor: String,
    pub brand_string: String,
}

impl CpuInfo {
    #[doc(hidden)]
    pub fn gather() -> Self {
        let cpuid = raw_cpuid::CpuId::new();
        Self {
            vendor: cpuid
                .get_vendor_info()
                .map(|v| v.as_str().to_string())
                .unwrap_or_default(),
            brand_string: cpuid
                .get_processor_brand_string()
                .map(|s| s.as_str().to_string())
                .unwrap_or_default(),
        }
    }
}

#[derive(Debug)]
pub struct SystemInfo {
    pub hostname: Option<String>,
    pub os: String,
    pub linux_distro: Option<String>,
    pub cpu_vendor: Option<String>,
    pub cpu_brand_string: Option<String>,
}

impl SystemInfo {
    #[doc(hidden)]
    pub fn gather() -> Self {
        let os = sys_info::os_type()
            .and_then(|ty| sys_info::os_release().map(|rel| (ty, rel)))
            .map(|(ty, rel)| format!("{ty} {rel}"))
            .unwrap_or("<failed to query OS information>".to_string());

        let linux_distro = sys_info::linux_os_release()
            .ok()
            .and_then(|r| r.pretty_name);

        let cpuid = raw_cpuid::CpuId::new();
        let cpu_vendor = cpuid.get_vendor_info().map(|v| v.as_str().to_string());
        let cpu_brand_string = cpuid
            .get_processor_brand_string()
            .map(|s| s.as_str().to_string());

        Self {
            hostname: sys_info::hostname().ok(),
            os,
            linux_distro,
            cpu_vendor,
            cpu_brand_string,
        }
    }

    pub fn log_debug(&self) {
        fn map_optional_string<'a>(s: &'a Option<String>) -> &'a str {
            s.as_ref().map(|s| s.as_str()).unwrap_or("<failed to get>")
        }

        tracing::event!(
            Level::DEBUG,
            hostname = map_optional_string(&self.hostname),
            os = self.os,
            linux_distro = map_optional_string(&self.linux_distro),
            cpu_vendor = map_optional_string(&self.cpu_vendor),
            cpu_brand_string = map_optional_string(&self.cpu_brand_string),
            "System information"
        )
    }
}

#[derive(Debug)]
pub struct RummageInfo {
    /// Information about the crate that contains the [`info!()`] invocation
    pub crate_info: CrateInfo,

    /// Information about the compilation process
    ///
    /// Specifically, this is information about how `rummage` itself was compiled. In Cargo's
    /// default configuration this will also be the same as the target crate being built, though it
    /// is possible to override this behavior in some circumstances, eg to build dependencies with
    /// the release profile even when the target crate is being built under the debug profile.
    pub compile_info: CompileInfo,

    /// Information about the system that the program is running on
    pub system_info: SystemInfo,

    /// The full command line that this executable was invoked with
    pub command_line: Vec<String>,

    /// Set of environment variables that have been explicitly gathered with
    /// [`RummageInfo::with_envvar`]/[`RummageInfo::with_envvars`].
    pub envvars: HashMap<String, Option<String>>,
}

impl RummageInfo {
    pub fn with_envvar(mut self, name: impl AsRef<str>) -> Self {
        let name = name.as_ref().to_string();
        let value = std::env::var(&name).ok();
        self.envvars.insert(name, value);
        self
    }

    pub fn with_envvars(mut self, names: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        for name in names {
            self = self.with_envvar(name);
        }
        self
    }

    /// Emits the contained information as [`tracing`] events, at the DEBUG level
    pub fn log_debug(self) {
        self.crate_info.log_debug();
        self.compile_info.log_debug();
        self.system_info.log_debug();

        tracing::event!(
            Level::DEBUG,
            args = format!("{:?}", self.command_line),
            "Command line args:"
        );

        tracing::event!(
            Level::DEBUG,
            args = format!("{:?}", self.envvars),
            "Collected environment variables"
        );
    }
}

#[macro_export]
macro_rules! info {
    () => {{
        ::rummage::RummageInfo {
            crate_info: ::rummage::_crate_info!(),
            compile_info: ::rummage::CompileInfo::gather(),
            system_info: ::rummage::SystemInfo::gather(),
            command_line: ::std::env::args().collect(),
            envvars: ::std::collections::HashMap::new(),
        }
    }};
}
