use std::{env, process};
use minigrep2::Config;

fn main() {
    let args = env::args();

    let config = Config::build(args)
        .unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        });

    minigrep2::run(config).unwrap_or_else(|err| {
        eprintln!("Application error: {err}");
        process::exit(1);
    });
}
