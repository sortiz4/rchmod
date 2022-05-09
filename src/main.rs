use rchmod::Rchmod;
use std::env;
use std::process;

fn main() {
    process::exit(run());
}

fn run() -> i32 {
    if let Ok(mut rchmod) = Rchmod::from_iter(env::args()) {
        if let Ok(_) = rchmod.run() {
            return 0;
        }
    }
    return 1;
}
