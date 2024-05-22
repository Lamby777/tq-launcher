#[cfg(feature = "cli")]
use clap::Parser;

use std::sync::OnceLock;

type Error = Box<dyn std::error::Error>;

pub(crate) static SILENT: OnceLock<bool> = OnceLock::new();

macro_rules! log {
    ($($arg:tt)*) => {
        if !SILENT.get().unwrap_or(&false) {
            eprintln!($($arg)*);
        }
    };
}

#[cfg(feature = "gui")]
pub fn gui_main() -> Result<(), Error> {
    log!("Loading...");
    Ok(())
}

#[cfg(feature = "cli")]
pub fn cli_main(opts: TqlOptions) -> Result<(), Error> {
    SILENT.set(opts.silent).unwrap();

    log!("Loading...");

    Ok(())
}

pub struct TqlOptions {
    silent: bool,
}

impl From<Args> for TqlOptions {
    fn from(args: Args) -> Self {
        Self {
            silent: args.silent,
        }
    }
}

#[cfg_attr(
    feature = "cli",
    derive(Parser, Debug),
    command(author, version, about)
)]
/// TerraQuest Launcher
pub struct Args {
    /// Silences progress "info" stderr messages.
    #[cfg_attr(feature = "cli", arg(short, long))]
    silent: bool,
}
