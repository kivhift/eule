//! Print command-line arguments.

use std::env;

fn main() {
    let mut args = env::args_os();

    if let Some(exe) = args.next() {
        println!("{}", exe.to_string_lossy());
    }

    let mut count: usize = 0;
    for arg in args {
        println!("  {}", arg.to_string_lossy());
        count += 1;
    }

    if count > 0 {
        println!("Supplied args: {count}");
    }
}
