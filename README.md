# Chmodrt
Chmodrt is a simple utility that recursively changes the mode of directories or
files and is only compatible with Unix systems. Numeric modes must be an octal
between one and four digits. Symbolic modes are not supported.

#### Usage

```
Recursively change the mode of files or directories.

Usage:
    chmodrt [OPTIONS] TYPE MODE PATHS

Options:
    -d, --dir           Change the mode of directories
    -D, --dry-run       Do not change any files (verbose)
    -f, --file          Change the mode of files
    -h, --help          Output this message
    -i, --interactive   Prompt before changing each file
    -s, --suppress      Suppress all interaction
    -v, --verbose       Explain what's being done
    -V, --version       Output version information
```

### Script
The previous version is available as a shell script, `chmodrt.sh`, and is
compatible with Windows through Cygwin and other similar distributions.
