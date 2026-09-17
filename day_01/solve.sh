#!/usr/bin/env bash

set -euo pipefail

input_source=${1:-"$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)/input.txt"}

# Read all input once so the same data can be used for both puzzle parts.
if [[ $input_source == "-" ]]; then
    mapfile -t lines
else
    mapfile -t lines < "$input_source"
fi

left=()
right=()
# Counting the right column up front makes each similarity lookup constant-time.
declare -A right_counts=()

# Split each row into its two IDs and retain both columns for later processing.
for line in "${lines[@]}"; do
    [[ -z $line ]] && continue
    read -r left_value right_value <<< "$line"
    left+=("$left_value")
    right+=("$right_value")
    ((right_counts["$right_value"] += 1))
done

mapfile -t sorted_left < <(printf '%s\n' "${left[@]}" | sort -n)
mapfile -t sorted_right < <(printf '%s\n' "${right[@]}" | sort -n)

# Part 1: pair the sorted IDs and add the absolute difference of each pair.
distance=0
for index in "${!sorted_left[@]}"; do
    difference=$((sorted_left[index] - sorted_right[index]))
    ((difference < 0)) && difference=$((-difference))
    distance=$((distance + difference))
done

similarity=0
# Part 2: each left ID contributes its value once for every matching right ID.
for value in "${left[@]}"; do
    similarity=$((similarity + value * right_counts["$value"]))
done

printf 'Part 1: %d\nPart 2: %d\n' "$distance" "$similarity"
