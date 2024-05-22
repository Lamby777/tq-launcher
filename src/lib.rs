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

pub fn cli_main(args: Args) -> Result<(), Error> {
    SILENT.set(args.silent).unwrap();

    log!("Loading...");

    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
/// TerraQuest Launcher
pub struct Args {
    /// Silences progress "info" stderr messages.
    #[arg(short, long)]
    silent: bool,
}
