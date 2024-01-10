fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = minigrep::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {err}");
        std::process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("application error: {e}");
        std::process::exit(1);
    }
}
