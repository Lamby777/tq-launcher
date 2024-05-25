use anyhow::Result;
use std::sync::OnceLock;

pub(crate) static SILENT: OnceLock<bool> = OnceLock::new();

macro_rules! log {
    ($($arg:tt)*) => {
        if !SILENT.get().unwrap_or(&false) {
            eprintln!($($arg)*);
        }
    };
}

pub fn run(opts: TqlOptions) -> Result<()> {
    SILENT.set(opts.silent).unwrap();

    log!("Loading...");

    Ok(())
}

pub struct TqlOptions {
    pub silent: bool,
}
