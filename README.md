# Chmodrt
Chmodrt is a simple utility that recursively changes the mode of directories or
files and runs wherever `chmod` is installed. Both numeric and symbolic modes
are supported.

## Usage
Changing files requires the `-f` option.

```
$ chmodrt -f MODE [PATHS]
```

Changing directories requires the `-d` option.

```
$ chmodrt -d MODE [PATHS]
```

Changing both requires the `-f` and `-d` option.

```
$ chmodrt -f MODE -d MODE [PATHS]
```
