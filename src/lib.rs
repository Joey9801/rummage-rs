#![doc = include_str!("../README.md")]

#[cfg(feature = "serde")]
use serde::Serialize;

use std::collections::HashMap;
use tracing::Level;

#[doc(hidden)]
pub use git_version::git_version;

/// Information about the crate that contains the [`info!`] invocation
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Debug)]
pub struct CrateInfo {
    /// The full SHA256 commit hash of the git repository
    pub git_commit_hash: String,

    /// Whether the git repository is in a dirty state / has uncommitted modifications
    pub is_git_repo_dirty: bool,

    /// The name of the binary that the crate that has the [`info!`] invocation is being built into
    pub bin_name: String,
    pub crate_name: String,
    pub crate_version: String,
}

impl CrateInfo {
    #[doc(hidden)]
    pub fn new(git_version: &str, crate_name: &str, crate_version: &str, bin_name: &str) -> Self {
        let dirty = git_version.ends_with("-dirty");
        let hash = git_version.trim_end_matches("-dirty");

        Self {
            git_commit_hash: hash.to_string(),
            is_git_repo_dirty: dirty,
            crate_name: crate_name.to_string(),
            crate_version: crate_version.to_string(),
            bin_name: bin_name.to_string(),
        }
    }

    fn log_debug(&self) {
        tracing::event!(
            Level::DEBUG,
            git_commit_hash = self.git_commit_hash,
            git_repo_dirty = self.is_git_repo_dirty,
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
        ::rummage::CrateInfo::new(
            ::rummage::git_version!(
                args = [
                    "--always",
                    "--abbrev=0",
                    "--match",
                    "NOT A TAG",
                    "--dirty=-dirty"
                ]
            ),
            option_env!("CARGO_CRATE_NAME").unwrap_or("<failed to scrape>"),
            option_env!("CARGO_PKG_VERSION").unwrap_or("<failed to scrape>"),
            option_env!("CARGO_BIN_NAME").unwrap_or("<failed to scrape>"),
        )
    }};
}

/// How Cargo was configured while building the crate containing the crate containing the [`info!`]
/// invocation
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Debug)]
pub struct CargoTarget {
    /// Typically either "debug" or "release"
    pub profile: String,

    /// The target triple of the environment performing the compilation
    pub host: String,

    /// The target triple of the environment the built artifact is intended for
    pub target: String,

    /// The "family" of the target, eg "unix"
    pub family: String,

    /// The specific OS within the target family, eg "linux"
    pub os: String,

    /// The CPU architecture of the target, eg "x86_64"
    pub arch: String,

    /// The number of bits in a pointer on the target platform, eg "64".
    pub pointer_width: String,

    /// The endianness of the target platform, eg "little"
    pub endian: String,

    /// A comma separated list of the features of the target platform that the compilation is
    /// using, eg "fxsr,sse,sse2"
    pub features: String,
}

impl CargoTarget {
    /// This method only works when called from within the rummage crate, as it relies on cargo
    /// environment variables rummage sets in its own build.rs
    fn gather() -> Self {
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

/// Details about the version of rustc that built this crate
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Debug)]
pub struct RustcVersion {
    pub rustc_semver: String,
    pub commit_hash: String,
    pub commit_date: String,
    pub llvm_version: String,
}

impl RustcVersion {
    /// This method only works when called from within the rummage crate, as it relies on cargo
    /// environment variables rummage sets in its own build.rs
    fn gather() -> Self {
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

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Debug)]
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

/// Runtime information about the system actually running the binary
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Debug)]
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
        fn map_optional_string(s: &Option<String>) -> &str {
            s.as_ref().map(|s| s.as_str()).unwrap_or("<failed to get>")
        }

        tracing::event!(
            Level::DEBUG,
            hostname = map_optional_string(&self.hostname),
            os = self.os,
            linux_distro = map_optional_string(&self.linux_distro),
            cpu_vendor = map_optional_string(&self.cpu_vendor),
            cpu_brand_string = map_optional_string(&self.cpu_brand_string),
            "System information:"
        )
    }
}

/// Top level info struct returned by [`info!`]
///
/// Example usage:
/// ```
/// rummage::info!()
///     .with_envvars(["RUST_LOG", "HOME", "MY_ENVVAR"])
///     .log_debug();
/// ```
#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Debug)]
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
    /// Enriches the content of this [`RummageInfo`] with the value of the given environment
    /// variable
    pub fn with_envvar(mut self, name: impl AsRef<str>) -> Self {
        let name = name.as_ref().to_string();
        let value = std::env::var(&name).ok();
        self.envvars.insert(name, value);
        self
    }

    /// Enrichves the content of this [`RummageInfo`] with the values of all the given environment
    /// variables
    pub fn with_envvars(mut self, names: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        for name in names {
            self = self.with_envvar(name);
        }
        self
    }

    /// Emits the contained information as [`tracing`] events, at the DEBUG level
    pub fn log_debug(&self) {
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
            "Environment variables:"
        );
    }
}

/// Build a [`RummageInfo`] struct containing all of the standard infomation sets
///
/// Needs to be a macro, as some information depends on which crate is actually executing the code.
/// If it were a regular function call, properties such as crate name/version would always just
/// refer to the build of `rummage` itself rather than the crate being instrumented.
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
