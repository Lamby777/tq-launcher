use tq_launcher::TqlOptions;

fn main() {
    let opts = TqlOptions { silent: false };

    if let Err(e) = tq_launcher::run_game(opts) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
