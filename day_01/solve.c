/*
 * Day 1 solved in C.
 *
 * Build: cc -O2 -o day_01/solve day_01/solve.c
 * Usage: ./day_01/solve [input_file]   (defaults to day_01/input.txt)
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* The input is a fixed 1000 lines, but grow the arrays anyway so the same
 * binary works on test_input.txt or any other input. */
typedef struct {
    long *items;
    size_t len;
    size_t cap;
} Vec;

/* Append one value, doubling the backing buffer whenever it fills up. */
static void vec_push(Vec *vec, long value) {
    if (vec->len == vec->cap) {
        vec->cap = vec->cap ? vec->cap * 2 : 16;
        vec->items = realloc(vec->items, vec->cap * sizeof(*vec->items));
        if (vec->items == NULL) {
            fprintf(stderr, "Out of memory\n");
            exit(1);
        }
    }

    vec->items[vec->len++] = value;
}

/* Comparator for qsort. Returning `a - b` would be the obvious thing to do,
 * but that can overflow, so compare instead. */
static int compare_longs(const void *a, const void *b) {
    long left = *(const long *)a;
    long right = *(const long *)b;

    if (left < right) {
        return -1;
    }

    return left > right;
}

/* Read the two whitespace-separated columns into `left` and `right`. */
static void parse_input(const char *path, Vec *left, Vec *right) {
    FILE *file = fopen(path, "r");
    if (file == NULL) {
        fprintf(stderr, "Failed to read input file: %s\n", path);
        exit(1);
    }

    /* " %ld %ld" skips leading whitespace (including newlines) and reads the
     * pair, so blank lines and the trailing newline take care of themselves.
     * fscanf returns the number of values it managed to read, so the loop
     * stops at EOF or on the first malformed line. */
    long l, r;
    while (fscanf(file, " %ld %ld", &l, &r) == 2) {
        vec_push(left, l);
        vec_push(right, r);
    }

    fclose(file);
}

/* Part 1: sort both columns, then sum the distance between each pair. */
static long part_01(const Vec *left, const Vec *right) {
    /* Sort copies so part 2 still sees the columns in their original,
     * line-by-line order. */
    long *sorted_left = malloc(left->len * sizeof(*sorted_left));
    long *sorted_right = malloc(right->len * sizeof(*sorted_right));
    if (sorted_left == NULL || sorted_right == NULL) {
        fprintf(stderr, "Out of memory\n");
        exit(1);
    }

    memcpy(sorted_left, left->items, left->len * sizeof(*sorted_left));
    memcpy(sorted_right, right->items, right->len * sizeof(*sorted_right));

    qsort(sorted_left, left->len, sizeof(*sorted_left), compare_longs);
    qsort(sorted_right, right->len, sizeof(*sorted_right), compare_longs);

    long sum = 0;
    for (size_t i = 0; i < left->len; i++) {
        long diff = sorted_left[i] - sorted_right[i];
        sum += diff < 0 ? -diff : diff;
    }

    free(sorted_left);
    free(sorted_right);

    return sum;
}

/* Part 2: sum each left-hand value multiplied by how many times it shows up in
 * the right-hand column. A hash map would be asymptotically better, but the
 * input is small enough that the nested loop finishes instantly. */
static long part_02(const Vec *left, const Vec *right) {
    long sum = 0;

    for (size_t i = 0; i < left->len; i++) {
        long occurrences = 0;

        for (size_t j = 0; j < right->len; j++) {
            if (right->items[j] == left->items[i]) {
                occurrences++;
            }
        }

        sum += left->items[i] * occurrences;
    }

    return sum;
}

int main(int argc, char **argv) {
    const char *input = argc > 1 ? argv[1] : "day_01/input.txt";

    Vec left = {0};
    Vec right = {0};
    parse_input(input, &left, &right);

    printf("part 1: %ld\n", part_01(&left, &right));
    printf("part 2: %ld\n", part_02(&left, &right));

    free(left.items);
    free(right.items);

    return 0;
}
