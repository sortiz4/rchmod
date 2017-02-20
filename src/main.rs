//! A simple program that recursively changes the mode of all directories or
//! files to the given mode under the given directory. The following child
//! process will be executed and waited on, and it's exit code returned
//! `sh -c "find PATH -type TYPE -exec chmod MODE {} \;"`.
use std::env;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process;
use std::process::Command;

// Successful execution
const ESUCCESS: i32 = 0x00;
// No exit code found
const ENOEXIT: i32 = 0x01;
// Invalid usage
const EUSAGE: i32 = 0x02;

// Shell arguments (compatibility wrapper)
const SHELL_ARGS: &'static [&'static str] = &[
    "sh", "-c"
];
// Chmod arguments (recursive typed chmod)
const CHMOD_ARGS: &'static [&'static str] = &[
    "find", "-type", "-exec", "chmod", "{}", "\\;"
];

// The program name
const NAME: &'static str = "chmodrt";
// The program usage
const USAGE: &'static str = "\
Usage: chmodrt TYPE MODE PATH\n\nTypes:
  -d  Change the mode of directories
  -f  Change the mode of files";

// Prepends the program name to the given message
macro_rules! formatsys {
    ($fmt:expr) => (format!(concat!("{}: ", $fmt), NAME));
    ($fmt:expr, $($arg:tt)*) => (format!(concat!("{}: ", $fmt), NAME, $($arg)*));
}

// Writes a formatted system message to the standard error
macro_rules! sys {
    ($fmt:expr) => (write!(&mut ::std::io::stderr(), "{}", formatsys!($fmt)));
    ($fmt:expr, $($arg:tt)*) => (write!(&mut ::std::io::stderr(), "{}", formatsys!($fmt, $($arg)*)));
}

// Writes a formatted system message to the standard error with a new line
macro_rules! sysln {
    ($fmt:expr) => (sys!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (sys!(concat!($fmt, "\n"), $($arg)*));
}

fn main() {
    process::exit(chmodrt(env::args().collect()));
}

fn chmodrt(args: Vec<String>) -> i32 {

    // Enforce correct usage
    if args.len() != 4 {
        println!("{}", USAGE);
        return EUSAGE;
    }

    // Check if the type is not a directory or a file
    if args[1].as_str() == "-d" || args[1].as_str() == "-f" {} else {
        sysln!("unknown type: '{}'", args[1]).unwrap();
        return EUSAGE;
    }

    // Authorize absolute paths
    if Path::new(args[3].as_str()).has_root() {
        let stdin_err = formatsys!("cannot read from stdin");
        loop {
            // The input buffer must be reset with every pass
            let mut input = String::new();

            // Print a confirmation prompt and wait for input
            sys!("path is absolute - continue? [y/n] ").unwrap();
            io::stdin().read_line(&mut input).expect(stdin_err.as_str());

            // Normalize the input for comparison
            input = input.trim().to_lowercase();
            let normal = input.as_str();

            // The response must be 'y' or 'n'
            if normal == "n" {
                sysln!("abort").unwrap();
                return ESUCCESS;
            } else if normal == "y" {
                break;
            }
        }
    }

    // Construct the child process and error messages
    let find_command = format!("{} {} {} {} {} {} {} {} {}", 
            CHMOD_ARGS[0], args[3], CHMOD_ARGS[1], args[1].trim_left_matches("-"),
            CHMOD_ARGS[2], CHMOD_ARGS[3], args[2], CHMOD_ARGS[4], CHMOD_ARGS[5]);
    let child_exec_err = formatsys!("failed to execute the child process: `{}`", SHELL_ARGS[0]);
    let child_wait_err = formatsys!("failed to wait on the child process: `{}`", SHELL_ARGS[0]);

    // Execute and wait on the child process
    let child = Command::new(SHELL_ARGS[0])
            .arg(SHELL_ARGS[1])
            .arg(find_command.as_str())
            .spawn().expect(child_exec_err.as_str())
            .wait().expect(child_wait_err.as_str());

    // Return the child exit code if it exists
    match child.code() {
        None => return ENOEXIT,
        Some(code) => return code,
    }
}
