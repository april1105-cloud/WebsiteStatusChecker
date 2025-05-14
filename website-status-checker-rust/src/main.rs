mod checker;
mod config;
mod result;
mod json_writer;

use crate::config::Config;
use crate::checker::run_checks;

fn main() {
    match Config::from_args() {
        Ok(config) => {
            if let Err(err) = run_checks(config) {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
        }
        Err(msg) => {
            eprintln!("Usage error: {}", msg);
            std::process::exit(2);
        }
    }
}
