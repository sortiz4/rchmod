//! Defines the options available.
use getopts::Matches;
use getopts::Options;
use super::status;
use super::text;

/// Used to define a new option flag.
pub struct Option<'a> {
    /// A short option (used with `-`).
    pub short: &'a str,
    /// A long option (used with `--`).
    pub long: &'a str,
    /// A brief description.
    pub description: &'a str,
}

/// Defines the 'dir' option flag.
pub const DIR: Option<'static> = Option {
    short: "d", long: "dir",
    description: "Change the mode of directories",
};

/// Defines the 'dry-run' option flag.
pub const DRYRUN: Option<'static> = Option {
    short: "D", long: "dry-run",
    description: "Do not change any files (verbose)",
};

/// Defines the 'file' option flag.
pub const FILE: Option<'static> = Option {
    short: "f", long: "file",
    description: "Change the mode of files",
};

/// Defines the 'help' option flag.
pub const HELP: Option<'static> = Option {
    short: "h", long: "help",
    description: "Output this message",
};

/// Defines the 'interactive' option flag.
pub const INTERACTIVE: Option<'static> = Option {
    short: "i", long: "interactive",
    description: "Prompt before changing each file",
};

/// Defines the 'suppress' option flag.
pub const SUPPRESS: Option<'static> = Option {
    short: "s", long: "suppress",
    description: "Suppress all interaction",
};

/// Defines the 'verbose' option flag.
pub const VERBOSE: Option<'static> = Option {
    short: "v", long: "verbose",
    description: "Explain what's being done",
};

/// Defines the 'version' option flag.
pub const VERSION: Option<'static> = Option {
    short: "V", long: "version",
    description: "Output version information",
};

/// Registers each option in the list.
macro_rules! optflags {
    ($opts:expr; $($name:ident),*) => {
        $($opts.optflag($name.short, $name.long, $name.description);)*
    };
}

/// Reformats the `getopts` error message.
macro_rules! reopt {
    ($var:expr) => ($var.to_string().to_lowercase().trim_right_matches(".").to_owned());
}

/// Appends the help string to the end of the given message.
macro_rules! help {
    ($fmt:expr) => (format!(concat!($fmt, "\n{}"), text::HELP));
    ($fmt:expr, $($arg:tt)*) => (format!(concat!($fmt, "\n{}"), $($arg)*, text::HELP));
}

/// Initializes a set of options from the option definitions.
pub fn create() -> Options {
    let mut options = Options::new();
    optflags![options; DIR, DRYRUN, FILE, HELP, INTERACTIVE, SUPPRESS, VERBOSE, VERSION];
    return options;
}

/// Parses a set of arguments into a set of matches.
pub fn parse(args: &Vec<String>, options: &Options) -> Result<Matches, String> {
    let matches = match options.parse(&args[1..]) {
        Ok(val) => val,
        Err(err) => return Err(help!("{}", reopt!(err))),
    };
    return Ok(matches);
}

/// Validate the set of matches.
pub fn validate(matches: &Matches) -> Result<(), String> {
    if matches.opt_present(INTERACTIVE.short) && matches.opt_present(SUPPRESS.short) {
        return Err(help!("{}: '{}', '{}'", status::MCONFLICT, INTERACTIVE.long, SUPPRESS.long));
    }
    if matches.opt_present(DIR.short) && matches.opt_present(FILE.short) {
        return Err(help!("{}: '{}', '{}'", status::MCONFLICT, DIR.long, FILE.long));
    }
    if !matches.opt_present(DIR.short) && !matches.opt_present(FILE.short) {
        return Err(help!("{}: '{}', '{}'", status::MNOTYPE, DIR.long, FILE.long));
    }
    if matches.free.len() < 2 {
        return Err(help!("{}", status::MFREEARGS));
    }
    return Ok(());
}
