#!/usr/bin/env bash
#
# Advent of Code 2024 - Day 1 (bash implementation)
#
# Usage: ./solution.sh [input_file]
#
# Defaults to input.txt next to this script.

set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
input_file="${1:-$script_dir/input.txt}"

if [[ ! -f "$input_file" ]]; then
    echo "Failed to read input file: $input_file" >&2
    exit 1
fi

left=()
right=()

while read -r l r || [[ -n "${l:-}" ]]; do
    if [[ -z "$l" ]]; then
        continue
    fi
    left+=("$l")
    right+=("$r")
done <"$input_file"

part_01() {
    local sorted_left sorted_right i diff sum=0

    mapfile -t sorted_left < <(printf '%s\n' "${left[@]}" | sort -n)
    mapfile -t sorted_right < <(printf '%s\n' "${right[@]}" | sort -n)

    for i in "${!sorted_left[@]}"; do
        diff=$((sorted_right[i] - sorted_left[i]))
        if ((diff < 0)); then
            diff=$((-diff))
        fi
        sum=$((sum + diff))
    done

    echo "$sum"
}

part_02() {
    local -A counts=()
    local r l sum=0

    for r in "${right[@]}"; do
        counts["$r"]=$((${counts["$r"]:-0} + 1))
    done

    for l in "${left[@]}"; do
        sum=$((sum + l * ${counts["$l"]:-0}))
    done

    echo "$sum"
}

echo "Part 1: $(part_01)"
echo "Part 2: $(part_02)"
