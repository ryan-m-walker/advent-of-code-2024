#!/usr/bin/env bash

set -euo pipefail

input_source=${1:-"$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)/input.txt"}

if [[ $input_source == "-" ]]; then
    mapfile -t lines
else
    mapfile -t lines < "$input_source"
fi

left=()
right=()
declare -A right_counts=()

for line in "${lines[@]}"; do
    [[ -z $line ]] && continue
    read -r left_value right_value <<< "$line"
    left+=("$left_value")
    right+=("$right_value")
    ((right_counts["$right_value"] += 1))
done

mapfile -t sorted_left < <(printf '%s\n' "${left[@]}" | sort -n)
mapfile -t sorted_right < <(printf '%s\n' "${right[@]}" | sort -n)

distance=0
for index in "${!sorted_left[@]}"; do
    difference=$((sorted_left[index] - sorted_right[index]))
    ((difference < 0)) && difference=$((-difference))
    distance=$((distance + difference))
done

similarity=0
for value in "${left[@]}"; do
    similarity=$((similarity + value * right_counts["$value"]))
done

printf 'Part 1: %d\nPart 2: %d\n' "$distance" "$similarity"
