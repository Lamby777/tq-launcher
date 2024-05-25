fn main() {
    if let Err(e) = tq_launcher::gui_main() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
