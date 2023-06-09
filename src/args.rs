use std::fmt::Display;
use std::path::PathBuf;

use clap::{ArgAction, ColorChoice, Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, ValueEnum, Default)]
pub enum Platform {
    #[cfg_attr(target_os = "linux", default)]
    Linux,
    #[value(name = "osx", alias = "macos")]
    #[cfg_attr(target_os = "macos", default)]
    OsX,
    #[cfg_attr(target_os = "windows", default)]
    Windows,
    Android,
    #[value(name = "sunos")]
    SunOs,
    #[cfg_attr(
        not(any(target_os = "linux", target_os = "macos", target_os = "windows")),
        default
    )]
    Common,
}

impl Platform {
    pub fn iterator() -> impl Iterator<Item = Platform> {
        use self::Platform::{Android, Linux, OsX, SunOs, Windows};
        [Linux, OsX, Windows, Android, SunOs].into_iter()
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Linux => "linux",
            Self::OsX => "osx",
            Self::Windows => "windows",
            Self::Android => "android",
            Self::SunOs => "sunos",
            Self::Common => "common",
        }
        .fmt(f)
    }
}

#[derive(Parser)]
#[command(
    arg_required_else_help = true,
    about,
    version = concat!('v', env!("CARGO_PKG_VERSION"), " (implementing the tldr client specification v1.5)"),
    disable_version_flag = true,
    after_help = "See 'man tldr' or https://acuteenvy.github.io/tlrc for more information.",
    help_template = "{before-help}{name} {version}\n\
    {about-with-newline}\n\
    {usage-heading} {usage}\n\n\
    {all-args}{after-help}"
)]
pub struct Cli {
    /// The tldr page to show.
    #[arg(group = "operations", required = true)]
    pub page: Vec<String>,

    /// Update the cache.
    #[arg(short, long, group = "operations")]
    pub update: bool,

    /// List all pages in the current platform.
    #[arg(short, long, group = "operations")]
    pub list: bool,

    /// List all pages.
    #[arg(short = 'a', long, group = "operations")]
    pub list_all: bool,

    /// Show cache information (installed languages and the number of pages).
    #[arg(short, long, group = "operations")]
    pub info: bool,

    /// Render the specified markdown file.
    #[arg(short, long, group = "operations", value_name = "FILE")]
    pub render: Option<PathBuf>,

    /// Clean the cache.
    #[arg(long, group = "operations")]
    pub clean_cache: bool,

    /// Print the default config.
    #[arg(long, group = "operations")]
    pub gen_config: bool,

    /// Print the default config path and create the config directory.
    #[arg(long, group = "operations")]
    pub config_path: bool,

    /// Specify the platform to use.
    #[arg(short, long, default_value_t = Platform::default())]
    pub platform: Platform,

    /// Specify the languages to use.
    #[arg(short = 'L', long = "language", value_name = "LANGUAGE")]
    pub languages: Option<Vec<String>>,

    /// Do not update the cache, even if it is stale.
    #[arg(short, long)]
    pub offline: bool,

    /// Strip empty lines from output.
    #[arg(short, long)]
    pub compact: bool,

    /// Do not strip empty lines from output (overrides --compact).
    #[arg(long)]
    pub no_compact: bool,

    /// Print pages in raw markdown instead of rendering them.
    #[arg(short = 'R', long)]
    pub raw: bool,

    /// Render pages instead of printing raw file contents (overrides --raw).
    #[arg(long)]
    pub no_raw: bool,

    /// Supress status messages.
    #[arg(short, long)]
    pub quiet: bool,

    /// Specify when to enable color.
    #[arg(long, value_name = "WHEN", default_value_t = ColorChoice::default())]
    pub color: ColorChoice,

    /// Specify an alternative path to the config file.
    #[arg(long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Print version.
    #[arg(short, long, action = ArgAction::Version)]
    version: (),
}
