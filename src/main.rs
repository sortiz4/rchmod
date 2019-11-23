use chmodrt::Chmodrt;
use std::env;
use std::process;

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
