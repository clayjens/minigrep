use clap::Parser;

fn main() {
    let config = minigrep::MinigrepCli::parse();

    if let Err(e) = minigrep::run(&config) {
        eprintln!("application error: {e}");
        std::process::exit(1);
    }
}
