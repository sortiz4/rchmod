# Chmodrt
Chmodrt is a simple utility that recursively changes the mode of directories or
files and is only compatible with Unix systems. Numeric modes must be an octal
between one and four digits. Symbolic modes are not supported.

## Usage
Changing files requires the `-f` option.

```
$ chmodrt -f MODE [PATHS]
```

Changing directories requires the `-d` option.

```
$ chmodrt -d MODE [PATHS]
```

## Script
The previous version is available as a shell script, `chmodrt.sh`, and is
compatible with Windows through Cygwin and other Unix-like environments.
