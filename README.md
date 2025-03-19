# Rchmod
`rchmod` is a command line utility that recursively changes file and directory
permissions throughout a directory tree. `rchmod` extends the functionality of
the standard `chmod` command with recursive and type capabilities making it
ideal for efficiently managing permissions across complex directory structures.

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
