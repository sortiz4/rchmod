use std::env;
use std::process;
use chmodrt::Chmodrt;

fn main() {
    process::exit(run());
}

fn run() -> i32 {
    if let Ok(mut chmodrt) = Chmodrt::from_iter(env::args()) {
        if let Ok(_) = chmodrt.run() {
            return 0;
        }
    }
    return 1;
}
