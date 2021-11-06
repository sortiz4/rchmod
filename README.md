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
A simplified version is also available as a shell script and is compatible with
other Unix-like environments.
