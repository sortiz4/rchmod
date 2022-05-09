use std::ffi::OsString;
use std::io;
use std::io::Stderr;
use std::io::Stdin;
use std::io::Stdout;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;
use super::Error;
use super::Result;

enum Context {
    /// The path is absolute (has root).
    Absolute,
    /// The `interactive` option is present.
    Interactive,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Recursively change the mode of files or directories.")]
struct Options {
    /// The mode to apply to files.
    #[structopt(short = "f", long = "file", name = "file-mode")]
    file: Option<String>,

    /// The mode to apply to directories.
    #[structopt(short = "d", long = "dir", name = "dir-mode")]
    dir: Option<String>,

    /// Do not overwrite any files (verbose).
    #[structopt(short = "D", long = "dry-run")]
    dry_run: bool,

    /// Prompt before overwriting each file.
    #[structopt(short = "i", long = "interactive")]
    interactive: bool,

    /// Suppress all interaction.
    #[structopt(short = "s", long = "suppress")]
    suppress: bool,

    /// Explain what's being done.
    #[structopt(short = "V", long = "verbose")]
    verbose: bool,

    /// Show this message.
    #[structopt(short = "h", long = "help")]
    help: bool,

    /// Show the version.
    #[structopt(short = "v", long = "version")]
    version: bool,

    /// The paths to be modified by this tool.
    #[structopt(name = "PATHS", parse(from_str))]
    paths: Vec<PathBuf>,
}

pub struct Chmodrt {
    options: Options,
    stderr: Stderr,
    stdout: Stdout,
    stdin: Stdin,
}

impl Chmodrt {
    /// Constructs this program from an iterable of arguments.
    pub fn from_iter<I>(iter: I) -> Result<Self>
    where
        Self: Sized,
        I: IntoIterator,
        I::Item: Into<OsString> + Clone,
    {
        return Ok(
            Self {
                options: Options::from_iter_safe(iter)?,
                stderr: io::stderr(),
                stdout: io::stdout(),
                stdin: io::stdin(),
            }
        );
    }

    /// Replaces the standard error stream for this program.
    pub fn stderr(&mut self, stderr: Stderr) -> &mut Self {
        self.stderr = stderr;
        return self;
    }

    /// Replaces the standard output stream for this program.
    pub fn stdout(&mut self, stdout: Stdout) -> &mut Self {
        self.stdout = stdout;
        return self;
    }

    /// Replaces the standard input stream for this program.
    pub fn stdin(&mut self, stdin: Stdin) -> &mut Self {
        self.stdin = stdin;
        return self;
    }

    /// Runs this program and writes all errors.
    pub fn run(&mut self) -> Result<()> {
        let mut run = || -> Result<()> {
            // Write the help or version message
            if self.options.help {
                return self.help();
            }
            if self.options.version {
                return self.version();
            }

            // Handle the paths
            return self.validate()?.change();
        };

        match run() {
            Ok(val) => {
                return Ok(val);
            },
            Err(err) => {
                writeln!(self.stderr, "Error: {}", err)?;
                return Err(err);
            },
        }
    }

    /// Validates the options.
    fn validate(&mut self) -> Result<&mut Self> {
        return if {
            self.options.interactive && self.options.suppress
        } {
            Err(Error::Conflict)
        } else if {
            !self.has_file() && !self.has_dir()
        } {
            Err(Error::Missing)
        } else {
            Ok(self)
        };
    }

    /// Writes the help message to the standard error stream.
    fn help(&mut self) -> Result<()> {
        Options::clap().write_help(&mut self.stderr)?;
        writeln!(self.stderr, "")?;
        return Ok(());
    }

    /// Writes the version message to the standard error stream.
    fn version(&mut self) -> Result<()> {
        Options::clap().write_version(&mut self.stderr)?;
        writeln!(self.stderr, "")?;
        return Ok(());
    }

    /// Authorizes directory and file access by prompting the user and reading
    /// from the standard input stream.
    fn authorize(&mut self, path: &PathBuf, context: Context) -> Result<bool> {
        // Determine the appropriate prompt
        let prompt = match context {
            Context::Absolute => "is absolute",
            Context::Interactive => "mode will be changed",
        };

        let mut input = String::new();
        loop {
            // Prompt the user and normalize the input
            write!(self.stderr, r#""{}" {} - continue? [y/n] "#, path.display(), prompt)?;
            self.stdin.read_line(&mut input)?;

            // The response must be `y` or `n`
            match input.trim().to_lowercase().as_str() {
                "n" => {
                    if self.options.verbose {
                        writeln!(self.stderr, "Skipped.")?;
                    }
                    return Ok(false);
                },
                "y" => {
                    return Ok(true);
                },
                _ => {
                    input.clear();
                },
            }
        }
    }

    /// Changes all paths provided by the user. Authorization may be requested
    /// if the `suppress` option is not present.
    fn change(&mut self) -> Result<()> {
        for path in self.options.paths.to_owned() {
            if !self.options.suppress && path.has_root() {
                // Authorize absolute paths (optional)
                if let Ok(false) = self.authorize(&path, Context::Absolute) {
                    continue;
                }
            }

            // Dig through directories
            if path.is_file() {
                self.change_one(&path)?;
            } else if path.is_dir() {
                self.change_many(&path)?;
            }
        }
        return Ok(());
    }

    /// Changes all entries under the given directory and writes all errors.
    fn change_many(&mut self, path: &PathBuf) -> Result<()> {
        let mut change_many = || -> Result<()> {
            // Change the children first
            for entry in path.read_dir()? {
                let path = entry?.path();

                // Dig through directories
                if path.is_file() {
                    self.change_one(&path)?;
                } else if path.is_dir() {
                    self.change_many(&path)?;
                }
            }

            // Change the parent last
            self.change_one(path)?;

            return Ok(());
        };

        return if let Err(err) = change_many() {
            self.write_error("Cannot access", path, &err)
        } else {
            Ok(())
        };
    }

    /// Changes the mode of the given path and writes all errors. Authorization
    /// may be requested and additional information may be written if the
    /// `interactive` and `verbose` options are present. The path will not be
    /// changed during a `dry-run`.
    fn change_one(&mut self, path: &PathBuf) -> Result<()> {
        let mut change_one = || -> Result<()> {
            if self.options.interactive && !self.options.suppress {
                // Authorize every path (optional)
                if let Ok(false) = self.authorize(path, Context::Interactive) {
                    return Ok(());
                }
            }

            if !self.options.dry_run {
                // Change the mode of the path
                let mode = if path.is_file() && self.has_file() {
                    self.options.file.as_ref().unwrap().to_owned()
                } else if path.is_dir() && self.has_dir() {
                    self.options.dir.as_ref().unwrap().to_owned()
                } else {
                    "".to_owned()
                };

                if mode.len() > 0 {
                    Command::new("chmod")
                        .args(&[mode, path.display().to_string()])
                        .status()
                        .unwrap();

                    if self.options.verbose {
                        // Write the results (optional)
                        self.write_result("changed", path)?;
                    }
                }
            } else {
                // Perform a dry run (optional)
                self.write_result("will be changed", path)?;
            }
            return Ok(());
        };

        return if let Err(err) = change_one() {
            self.write_error("Cannot change permissions", path, &err)
        } else {
            Ok(())
        };
    }

    /// Determines if the file option is present.
    fn has_file(&self) -> bool {
        return self.options.file.is_some();
    }

    /// Determines if the directory option is present.
    fn has_dir(&self) -> bool {
        return self.options.dir.is_some();
    }

    /// Writes a path related error to the standard error stream.
    fn write_error(&mut self, msg: &str, path: &PathBuf, err: &Error) -> Result<()> {
        writeln!(self.stderr, r#"Error: {} "{}": {}"#, msg, path.display(), err)?;
        return Ok(());
    }

    /// Writes the result of an operation to the standard output stream.
    fn write_result(&mut self, msg: &str, path: &PathBuf) -> Result<()> {
        writeln!(self.stdout, r#""{}": mode {}."#, path.display(), msg)?;
        return Ok(());
    }
}
