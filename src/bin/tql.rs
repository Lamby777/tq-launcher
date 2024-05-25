use clap::Parser;
use tq_launcher::TqlOptions;

#[derive(Parser, Debug)]
#[command(author, version, about)]
/// TerraQuest Launcher
pub struct Args {
    /// Silences progress "info" stderr messages.
    #[arg(short, long)]
    silent: bool,
}

fn main() {
    let args = Args::parse();
    let opts = TqlOptions {
        silent: args.silent,
    };

    if let Err(e) = tq_launcher::run(opts) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
