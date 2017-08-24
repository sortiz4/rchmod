//! Defines the core functionality.
use getopts::Matches;
use std::fs;
use std::fs::Permissions;
use std::io;
use std::io::Result;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;
use super::opts;
use super::status;
use super::text;

/// Named constants used to indicate the authorization context.
pub enum Auth {
    /// The path is absolute (has root).
    Absolute,
    /// The 'interactive' option is present.
    Interactive,
}

/// Authorizes directory and file access by prompting the user and reading the
/// standard input. `true` will be returned if the user grants permission and
/// `false` will be returned otherwise.
/// # Issues
/// - If the standard input is closed or empty, no error will be raised and the
/// loop will continue indefinitely.
pub fn auth(path: &Path, auth: Auth) -> bool {
    let stdin_err = formats!("{}", status::MSTDINERR);
    let mut input = String::new();

    // Determine the appropriate prompt
    let prompt = match auth {
        Auth::Absolute => text::ABSOLUTE,
        Auth::Interactive => text::INTERACTIVE,
    };

    loop {
        // Prompt the user and normalize the input
        sprint!("'{}' {} {} ", path.display(), prompt, text::CONTINUE);
        io::stdin().read_line(&mut input).expect(&stdin_err);
        input = input.trim().to_lowercase();

        // The response must be YES or NO
        match input.as_str() {
            text::YES => return true,
            text::NO => {
                sprintln!("{}", text::SKIP);
                return false;
            },
            _ => {
                input.clear();
                continue;
            },
        }
    }
}

/// Adds all files *under* the given directory to the `list`.
pub fn collect_files(dir: &Path, list: &mut Vec<PathBuf>) -> Result<()> {
    // Iterate over all entries in the directory
    for entry in dir.read_dir()? {
        let path = entry?.path();
        // Recurse if the entry is a directory,
        // otherwise add the entry to the list
        if path.is_dir() {
            collect_files(&path, list)?;
        } else if path.is_file() {
            list.push(path);
        }
    }
    return Ok(());
}

/// Adds all directories *under* the given directory (including `dir`) to the `list`.
pub fn collect_dirs(dir: &Path, list: &mut Vec<PathBuf>) -> Result<()> {
    list.push(dir.to_owned());
    // Iterate over all entries in the directory
    for entry in dir.read_dir()? {
        let path = entry?.path();
        // Recurse if the entry is a directory
        if path.is_dir() {
            collect_dirs(&path, list)?;
        }
    }
    return Ok(());
}

/// Changes the mode of the given path. Authorization may be requested and
/// additional information may be printed if the 'interactive' and 'verbose'
/// options are present. The file will not be changed during a 'dry-run'.
pub fn chmod_one(mode: u32, path: &Path, matches: &Matches) -> Result<()> {
    // Authorize every file (optional)
    if matches.opt_present(opts::INTERACTIVE.short) && !matches.opt_present(opts::SUPPRESS.short) {
        if let false = auth(&path, Auth::Interactive) {
            return Ok(());
        }
    }

    // Change the file's mode or perform a dry run (optional)
    if !matches.opt_present(opts::DRYRUN.short) {
        fs::set_permissions(path, Permissions::from_mode(mode))?;

        // Print the result to the standard output (optional)
        if matches.opt_present(opts::VERBOSE.short) {
            println!("'{}': {}", path.display(), text::CHANGE);
        }
    } else {
        println!("'{}': {}", path.display(), text::DRYRUN);
    }
    return Ok(());
}

/// Calls `chmod_one` for each file in the `list` and handles all
/// associated errors internally.
pub fn chmod_many(mode: u32, list: &Vec<PathBuf>, matches: &Matches) {
    for file in list.iter() {
        if let Err(err) = chmod_one(mode, file, matches) {
            sprintln!("{} '{}': {}", status::MACCESS, file.display(), err);
            continue;
        }
    }
}
