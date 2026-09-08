#!/usr/bin/env bash
#
# Day 1 solved in bash.
#
# Usage: ./solve.sh [input_file]   (defaults to ./input.txt)

# Exit on error, on unset variables, and on failures inside a pipeline.
set -euo pipefail

# Input file comes from the first argument, otherwise input.txt next to this
# script (so it works no matter which directory you run it from).
input="${1:-"$(dirname "$0")/input.txt"}"

if [[ ! -f "$input" ]]; then
    echo "Failed to read input file: $input" >&2
    exit 1
fi

# The two columns of the input, plus how many times each right-hand value
# appears (part 2 needs those counts, so they're tallied during parsing).
left=()
right=()
declare -A right_counts=()

# Each line is two numbers separated by whitespace. `read` splits on $IFS, so
# `l` and `r` pick up the two columns and the trailing `_` soaks up anything
# else on the line.
while read -r l r _; do
    # Skip blank lines (including a trailing newline at the end of the file).
    [[ -z "$l" ]] && continue
    left+=("$l")
    right+=("$r")
    # `:-0` defaults the count to 0 the first time we see a value.
    right_counts["$r"]=$((${right_counts["$r"]:-0} + 1))
done <"$input"

# Part 1: sort both columns, then sum the distance between each pair.
part_01() {
    local -a sorted_left sorted_right
    # Bash has no sort builtin, so shell out to `sort -n` (numeric, otherwise
    # "10" would sort before "9") and read the lines back into an array.
    mapfile -t sorted_left < <(printf '%s\n' "${left[@]}" | sort -n)
    mapfile -t sorted_right < <(printf '%s\n' "${right[@]}" | sort -n)

    local sum=0 i diff
    for i in "${!sorted_left[@]}"; do
        diff=$((sorted_left[i] - sorted_right[i]))
        # `${diff#-}` strips a leading minus sign, i.e. absolute value.
        sum=$((sum + ${diff#-}))
    done

    echo "$sum"
}

# Part 2: sum each left-hand value multiplied by how many times it shows up in
# the right-hand column.
part_02() {
    local sum=0 l
    for l in "${left[@]}"; do
        # Values missing from the right column count as 0 and contribute nothing.
        sum=$((sum + l * ${right_counts["$l"]:-0}))
    done

    echo "$sum"
}

echo "part 1: $(part_01)"
echo "part 2: $(part_02)"
