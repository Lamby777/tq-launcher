use clap::Parser;
// use tq_launcher::TqlOptions;

#[derive(Parser, Debug)]
#[command(author, version, about)]
/// TerraQuest Launcher
pub struct Args {
    /// The action to perform.
    action: Option<String>,

    /// Silences progress "info" stderr messages.
    #[arg(short, long)]
    silent: bool,
}

fn main() {
    todo!()
    // let args = Args::parse();
    // let opts = TqlOptions {
    //     silent: args.silent,
    // };
    //
    // if let Err(e) = tq_launcher::run(opts) {
    //     eprintln!("Error: {}", e);
    //     std::process::exit(1);
    // }
}
