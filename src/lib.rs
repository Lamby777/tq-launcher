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

#[cfg(feature = "cli")]
pub fn cli_main(opts: TqlOptions) -> Result<(), Error> {
    SILENT.set(opts.silent).unwrap();

    log!("Loading...");

    Ok(())
}

pub struct TqlOptions {
    pub silent: bool,
}
