#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_LINE 1024
#define MAX_LEVELS 64

/* Parses a whitespace separated line of numbers into levels, returning the
 * count. */
static int parse_line(const char *line, int *levels) {
    int count = 0;
    const char *cursor = line;

    while (*cursor != '\0') {
        char *end;
        long value = strtol(cursor, &end, 10);

        if (end == cursor) {
            cursor++;
            continue;
        }

        if (count == MAX_LEVELS) {
            fprintf(stderr, "too many levels on a single line\n");
            exit(EXIT_FAILURE);
        }

        levels[count++] = (int)value;
        cursor = end;
    }

    return count;
}

/* A report is safe when every step moves in the same direction by 1 to 3. */
static bool is_safe(const int *levels, int count) {
    bool desc = false;

    for (int i = 0; i + 1 < count; i++) {
        int diff = levels[i] - levels[i + 1];
        int abs_diff = diff < 0 ? -diff : diff;

        if (abs_diff < 1 || abs_diff > 3) {
            return false;
        }

        if (i == 0) {
            desc = diff > 0;
        } else if (desc != (diff > 0)) {
            return false;
        }
    }

    return true;
}

/* Safe once any single level is allowed to be dropped. */
static bool is_safe_dampened(const int *levels, int count) {
    if (is_safe(levels, count)) {
        return true;
    }

    for (int skip = 0; skip < count; skip++) {
        int filtered[MAX_LEVELS];
        int filtered_count = 0;

        for (int i = 0; i < count; i++) {
            if (i != skip) {
                filtered[filtered_count++] = levels[i];
            }
        }

        if (is_safe(filtered, filtered_count)) {
            return true;
        }
    }

    return false;
}

int main(int argc, char **argv) {
    const char *path = argc > 1 ? argv[1] : "example_input.txt";
    FILE *file = fopen(path, "r");

    if (file == NULL) {
        fprintf(stderr, "failed to open %s\n", path);
        return EXIT_FAILURE;
    }

    char line[MAX_LINE];
    int part_1 = 0;
    int part_2 = 0;

    while (fgets(line, sizeof(line), file) != NULL) {
        int levels[MAX_LEVELS];
        int count = parse_line(line, levels);

        if (count == 0) {
            continue;
        }

        if (is_safe(levels, count)) {
            part_1++;
        }

        if (is_safe_dampened(levels, count)) {
            part_2++;
        }
    }

    fclose(file);

    printf("part 1: %d\n", part_1);
    printf("part 2: %d\n", part_2);

    return EXIT_SUCCESS;
}
