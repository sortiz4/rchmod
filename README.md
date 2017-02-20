# Chmodrt
A simple program that recursively changes the mode of all directories or files
to the given mode under the given directory. The following child process will
be executed and waited on, and it's exit code returned
`sh -c "find PATH -type TYPE -exec chmod MODE {} \;"`.

#### Usage

```
Usage: chmodrt TYPE MODE PATH

Types:
  -d  Change the mode of directories
  -f  Change the mode of files
```
