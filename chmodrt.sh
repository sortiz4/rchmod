#!/bin/sh
name="chmodrt"

# Validate the arguments (count only)
if [ "$#" -lt 3 ]; then
    printf "Usage: $name TYPE MODE PATH\n\n"
    printf "Types:\n"
    printf "    -d  Change the mode of directories\n"
    printf "    -f  Change the mode of files\n"
    exit 1
fi

# Authorize absolute paths (all or nothing)
for path in "${@:3}"; do
    if [[ "$path" = /* ]] || [[ "$path" = [A-Za-z]:/* ]]; then
        while true; do
            read -p "$name: $path is absolute - continue? [y/n] " yn
            case $yn in
                [Yy]* ) break;;
                [Nn]* ) printf "$name: abort\n"; exit 0;;
            esac
        done
    fi
done

# Evaluate the type and execute
type=`printf -- $1 | sed "s/-//g"`
find ${@:3} -type $type -exec chmod $2 {} \;
