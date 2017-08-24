#[macro_use]
extern crate chmodrt;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use chmodrt::core;
use chmodrt::core::Auth;
use chmodrt::opts;
use chmodrt::status;
use chmodrt::text;

fn main() {
    process::exit(chmodrt(env::args().collect()));
}

fn chmodrt(args: Vec<String>) -> i32 {
    // Parse the command line options
    let options = opts::create();
    let matches = match opts::parse(&args, &options) {
        Ok(val) => val,
        Err(err) => {
            sprintln!("{}", err);
            return status::EUSAGE;
        },
    };

    // Display the help message or version and exit (optional)
    if matches.opt_present(opts::HELP.short) {
        print!("{}", options.usage(text::USAGE));
        return status::ESUCCESS;
    } else if matches.opt_present(opts::VERSION.short) {
        println!("{} {}", text::NAME, text::VERSION);
        return status::ESUCCESS;
    }

    // Validate the options
    if let Err(err) = opts::validate(&matches) {
        sprintln!("{}", err);
        return status::EUSAGE;
    }

    // Parse the mode string
    let mode = match parse_mode(&matches.free[0]) {
        Ok(val) => val,
        Err(_) => {
            sprintln!("{}: {}", matches.free[0], status::MNUMERIC);
            return status::EUSAGE;
        }
    };

    // Loop through the free arguments (paths)
    let mut list: Vec<PathBuf> = Vec::new();
    for item in matches.free[1..].iter() {
        let path = Path::new(item);

        // Authorize absolute paths (optional)
        if !matches.opt_present(opts::SUPPRESS.short) && path.has_root() {
            if let false = core::auth(&path, Auth::Absolute) {
                continue;
            }
        }

        // Verify that the path exists in the file system
        if path.exists() {
            // Attempt to collect directories or files (type option)
            if matches.opt_present(opts::DIR.short) && path.is_dir() {
                if let Err(err) = core::collect_dirs(&path, &mut list) {
                    sprintln!("{} '{}': {}", status::MACCESS, item, err);
                    continue;
                }
            } else if matches.opt_present(opts::FILE.short) {
                // Collect all files under the given directories
                if path.is_dir() {
                    if let Err(err) = core::collect_files(&path, &mut list) {
                        sprintln!("{} '{}': {}", status::MACCESS, item, err);
                        continue;
                    }
                } else if path.is_file() {
                    list.push(path.to_owned());
                }
            }
        } else {
            sprintln!("'{}' {}", item, status::MNOTFOUND);
        }

        // Change the mode of each path in the list
        if list.len() > 0 {
            core::chmod_many(mode, &list, &matches);
            list.clear(); // Truncate the list
        }
    }
    return status::ESUCCESS;
}

fn parse_mode(arg: &str) -> Result<u32, ()> {
    if arg.len() > 4 {
        return Err(());
    }
    let mode = match u32::from_str_radix(arg, 8) {
        Ok(val) => val,
        Err(_) => return Err(()),
    };
    return Ok(mode);
}
