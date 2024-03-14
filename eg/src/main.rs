//! Print environment variables that match a given regex.
use std::{env, process::ExitCode};

use clap::Parser;
use regex::Regex;

#[derive(Parser)]
#[clap(version, about = "Search your environment for a given regex")]
struct Args {
    #[arg(short, long, help = "Search values instead of variables")]
    values: bool,
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
    if args.values {
        for (key, value) in env::vars() {
            if re.is_match(&value) {
                println!("{key}={value}");
                found = true;
            }
        }
    } else {
        for (key, value) in env::vars() {
            if re.is_match(&key) {
                println!("{key}={value}");
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
