/// The program name.
#[macro_export]
macro_rules! name {
    () => ("chmodrt");
}

/// Help message indentation.
#[macro_export]
macro_rules! indent {
    ($arg:expr) => (concat!["    ", $arg]);
    ($fmt:expr, $($arg:tt)*) => (indent![concat![$fmt, $(" ", $arg)*]]);
}

/// Prepends the program name to the given message.
#[macro_export]
macro_rules! formats {
    ($fmt:expr) => (format!(concat![name![], ": ", $fmt]));
    ($fmt:expr, $($arg:tt)*) => (format!(concat![name![], ": ", $fmt], $($arg)*));
}

/// Writes a formatted system message to the standard error.
#[macro_export]
macro_rules! sprint {
    ($fmt:expr) => (eprint!(concat![name![], ": ", $fmt]));
    ($fmt:expr, $($arg:tt)*) => (eprint!(concat![name![], ": ", $fmt], $($arg)*));
}

/// Writes a formatted system message to the standard error with a new line.
#[macro_export]
macro_rules! sprintln {
    () => (sprint!("\n"));
    ($fmt:expr) => (sprint!(concat![$fmt, "\n"]));
    ($fmt:expr, $($arg:tt)*) => (sprint!(concat![$fmt, "\n"], $($arg)*));
}
