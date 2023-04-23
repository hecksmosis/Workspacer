use std::process;
use workspace_mgr::{configuration::Config, errors::Severity};
use clap::Parser;

fn main() {
    // Get the command line arguments
    let config = Config::parse();

    // Run the application
    if let Err(e) = workspace_mgr::run(config) {
        if e.severity == Severity::Error {
            eprintln!("Error: {e}");
            process::exit(1);
        } else if e.severity == Severity::Warning {
            eprintln!("Warning: {e}");
        } else {
            eprintln!("{e}");
        }
        process::exit(1);
    }
}
