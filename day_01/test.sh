#!/bin/sh

set -eu

compiler=${CC:-cc}
binary=$(mktemp)
output=$(mktemp)
trap 'rm -f "$binary" "$output"' EXIT

"$compiler" -std=c11 -Wall -Wextra -Werror day_01.c -o "$binary"
"$binary" test_input.txt > "$output"

expected='Part 1: 11
Part 2: 31'
actual=$(cat "$output")

if [ "$actual" != "$expected" ]; then
    echo "Unexpected output:" >&2
    cat "$output" >&2
    exit 1
fi

echo "Day 1 example passed."
