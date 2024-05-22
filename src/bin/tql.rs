use clap::Parser;
use tq_launcher::Args;

fn main() {
    let args = Args::parse();

    if let Err(e) = tq_launcher::cli_main(args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
