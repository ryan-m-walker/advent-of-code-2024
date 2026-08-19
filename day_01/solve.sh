#!/usr/bin/env bash
#
# Day 1 solved in bash.
#
# Usage: ./solve.sh [input_file]   (defaults to ./input.txt)

set -euo pipefail

input="${1:-"$(dirname "$0")/input.txt"}"

if [[ ! -f "$input" ]]; then
    echo "Failed to read input file: $input" >&2
    exit 1
fi

left=()
right=()
declare -A right_counts=()

while read -r l r _; do
    [[ -z "$l" ]] && continue
    left+=("$l")
    right+=("$r")
    right_counts["$r"]=$((${right_counts["$r"]:-0} + 1))
done <"$input"

part_01() {
    local -a sorted_left sorted_right
    mapfile -t sorted_left < <(printf '%s\n' "${left[@]}" | sort -n)
    mapfile -t sorted_right < <(printf '%s\n' "${right[@]}" | sort -n)

    local sum=0 i diff
    for i in "${!sorted_left[@]}"; do
        diff=$((sorted_left[i] - sorted_right[i]))
        sum=$((sum + ${diff#-}))
    done

    echo "$sum"
}

part_02() {
    local sum=0 l
    for l in "${left[@]}"; do
        sum=$((sum + l * ${right_counts["$l"]:-0}))
    done

    echo "$sum"
}

echo "part 1: $(part_01)"
echo "part 2: $(part_02)"
