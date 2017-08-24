//! Defines common string fragments.

/// The program name.
pub const NAME: &str = name![];
/// The program version.
pub const VERSION: &str = "2.0";
/// A generic help message.
pub const HELP: &str = concat![
    "Try '", name![], " --help' for more information.",
];
/// A brief description of the program and its usage.
pub const USAGE: &str = concat![
    "Recursively change the mode of files or directories.\n\n",
    "Usage:\n", indent![name![], "[OPTIONS] TYPE MODE PATHS"],
];

/// A descriptor that maps to the `Auth::Absolute` context.
pub const ABSOLUTE: &str = "is absolute";
/// A descriptor that maps to the `Auth::Interactive` context.
pub const INTERACTIVE: &str = "mode will be changed";

/// A descriptor that maps to the 'dry-run' option.
pub const DRYRUN: &str = "mode will be changed";
/// A descriptor that maps to the 'verbose' option.
pub const CHANGE: &str = "mode changed";

/// A continue prompt.
pub const CONTINUE: &str = "- continue? [y/n]";
/// Indicates an action has been skipped.
pub const SKIP: &str = "skipped";
/// Represents a 'yes' response.
pub const YES: &str = "y";
/// Represents a 'no' response.
pub const NO: &str = "n";
