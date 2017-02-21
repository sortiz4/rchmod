//! A simple program that recursively changes the mode of all directories or
//! files to the given mode under the given directory. The following child
//! process will be executed and waited on, and it's exit code returned
//! `sh -c "find PATH -type TYPE -exec chmod MODE {} \;"`.
use std::env;
use std::fmt::Write as FmtWrite;
use std::io;
use std::io::Write as IoWrite;
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
const USAGE: &'static str = "Usage: chmodrt TYPE MODE PATH";
// The program options
const TYPES: &'static [&'static [&'static str]] = &[
    &["-d", "Change the mode of directories"],
    &["-f", "Change the mode of files"],
];

// Prepends the program name to the given message
macro_rules! format_sys {
    ($fmt:expr) => (format!(concat!("{}: ", $fmt), NAME));
    ($fmt:expr, $($arg:tt)*) => (format!(concat!("{}: ", $fmt), NAME, $($arg)*));
}

// Writes a formatted system message to the standard error
macro_rules! sys {
    ($fmt:expr) => (write!(&mut ::std::io::stderr(), "{}", format_sys!($fmt)));
    ($fmt:expr, $($arg:tt)*) => (write!(&mut ::std::io::stderr(), "{}", format_sys!($fmt, $($arg)*)));
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
        print!("{}\n\n{}", USAGE, options());
        return EUSAGE;
    }

    // Check if the type is not a directory or a file
    if args[1] == TYPES[0][0] || args[1] == TYPES[1][0] {} else {
        sysln!("unknown type: '{}'", args[1]).unwrap();
        return EUSAGE;
    }

    // Authorize absolute paths
    if Path::new(&args[3]).has_root() {
        let stdin_err = format_sys!("cannot read from stdin");
        loop {
            // The input buffer must be reset with every pass
            let mut input = String::new();

            // Print a confirmation prompt and wait for input
            sys!("path is absolute - continue? [y/n] ").unwrap();
            io::stdin().read_line(&mut input).expect(&stdin_err);

            // Normalize the input for comparison
            input = input.trim().to_lowercase();

            // The response must be 'y' or 'n'
            match input.as_str() {
                "y" => {
                    break;
                },
                "n" => {
                    sysln!("abort").unwrap();
                    return ESUCCESS;
                },
                _ => continue,
            }
        }
    }

    // Construct the child process and error messages
    let find_command = format!("{} {} {} {} {} {} {} {} {}",
                               CHMOD_ARGS[0], args[3], CHMOD_ARGS[1], args[1].trim_left_matches("-"),
                               CHMOD_ARGS[2], CHMOD_ARGS[3], args[2], CHMOD_ARGS[4], CHMOD_ARGS[5]);
    let child_exec_err = format_sys!("failed to execute the child process: `{}`", SHELL_ARGS[0]);
    let child_wait_err = format_sys!("failed to wait on the child process: `{}`", SHELL_ARGS[0]);

    // Execute and wait on the child process
    let child = Command::new(SHELL_ARGS[0])
        .arg(SHELL_ARGS[1])
        .arg(find_command)
        .spawn().expect(&child_exec_err)
        .wait().expect(&child_wait_err);

    // Return the child exit code if it exists
    match child.code() {
        None => return ENOEXIT,
        Some(code) => return code,
    }
}

fn options() -> String {

    // Initialize the buffer and write the options header
    let mut buffer = String::with_capacity(128);
    writeln!(&mut buffer, "Types:").unwrap();

    // Enumerate the options and return the buffer
    for outer in TYPES.iter() {
        for inner in outer.iter() {
            write!(&mut buffer, "{:2}{}", "", inner).unwrap();
        }
        writeln!(&mut buffer, "").unwrap();
    }
    return buffer;
}
