//! Print executables on $PATH that match a given regex.
use std::{env, process::ExitCode};

use clap::Parser;
use is_executable::IsExecutable;
use regex::Regex;

#[derive(Parser)]
#[clap(
    version,
    about = "Search your $PATH for an executable that matches a regex"
)]
struct Args {
    #[arg(short, long, help = "Complain about non-extant directories in $PATH")]
    complain: bool,
    #[arg(help = "Regex to search for")]
    regex: Option<String>,
}

fn main() -> ExitCode {
    let args = Args::parse();

    if args.regex.is_none() {
        return ExitCode::SUCCESS;
    }

    let re = Regex::new(&args.regex.unwrap()).expect("Failed to compile regex");
    let mut found = false;
    for dir in env::split_paths(&env::var_os("PATH").expect("$PATH is not set")) {
        if !dir.is_dir() {
            if args.complain {
                eprintln!("Not a directory: {}", dir.display());
            }
            continue;
        }

        for entry in dir.read_dir().unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if !path.is_executable() {
                continue;
            }
            let stem = path.file_stem().unwrap();
            if re.is_match(&stem.to_string_lossy()) {
                println!("{}", path.display());
                found = true;
            }
        }
    }

    if found {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
