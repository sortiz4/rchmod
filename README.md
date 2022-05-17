# Rchmod
Rchmod is a simple command line tool that recursively changes the mode of
directories or files and supports all modes parsable by chmod.

## Usage
Changing files requires the `-f` option.

```
$ rchmod -f MODE [PATHS]
```

Changing directories requires the `-d` option.

```
$ rchmod -d MODE [PATHS]
```

Changing both requires the `-f` and `-d` option.

```
$ rchmod -f MODE -d MODE [PATHS]
```
