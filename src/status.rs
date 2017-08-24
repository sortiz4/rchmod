//! Defines common exit codes (`E`) and error messages (`M`).

/// Successful execution.
pub const ESUCCESS: i32 = 0x00;
/// Invalid usage.
pub const EUSAGE: i32 = 0x01;

/// The file or directory cannot be accessed.
pub const MACCESS: &str = "cannot access";
/// A usage error where the type is missing.
pub const MNOTYPE: &str = "missing type option";
/// The file or directory cannot be found.
pub const MNOTFOUND: &str = "cannot not be found";
/// A usage error where conflicting options are present.
pub const MCONFLICT: &str = "conflicting options";
/// The program cannot read from the standard input.
pub const MSTDINERR: &str = "cannot read from stdin";
/// A usage error where the number of free arguments is too small.
pub const MFREEARGS: &str = "a mode and at least one path must be provided";
/// A usage error where the numeric mode is invalid.
pub const MNUMERIC: &str = "mode must be an octal between one and four digits";
