use std::{env, process};
use minigrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args)
        .unwrap_or_else(|err: &str| {
            eprintln!("Problem parsing arguments: \n{err}");
            process::exit(1);
        });

    let application_result = minigrep::run(config);
    if let Err(err) = application_result {
        eprintln!("Application error: \n{err}");
        process::exit(1);
    }
}