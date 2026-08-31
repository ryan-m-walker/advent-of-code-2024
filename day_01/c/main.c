#include <stdio.h>
#include <stdlib.h>

#define MAX_LINES 2048

static int compare_ints(const void *a, const void *b) {
    int left = *(const int *)a;
    int right = *(const int *)b;

    return (left > right) - (left < right);
}

static int parse_input(const char *path, int *left, int *right) {
    FILE *file = fopen(path, "r");

    if (file == NULL) {
        fprintf(stderr, "Failed to read input file: %s\n", path);
        exit(EXIT_FAILURE);
    }

    int count = 0;

    while (count < MAX_LINES && fscanf(file, "%d %d", &left[count], &right[count]) == 2) {
        count++;
    }

    fclose(file);

    return count;
}

static long part_01(const int *left, const int *right, int count) {
    int *sorted_left = malloc((size_t)count * sizeof(int));
    int *sorted_right = malloc((size_t)count * sizeof(int));

    if (sorted_left == NULL || sorted_right == NULL) {
        fprintf(stderr, "Failed to allocate memory\n");
        exit(EXIT_FAILURE);
    }

    for (int i = 0; i < count; i++) {
        sorted_left[i] = left[i];
        sorted_right[i] = right[i];
    }

    qsort(sorted_left, (size_t)count, sizeof(int), compare_ints);
    qsort(sorted_right, (size_t)count, sizeof(int), compare_ints);

    long sum = 0;

    for (int i = 0; i < count; i++) {
        sum += labs(sorted_right[i] - sorted_left[i]);
    }

    free(sorted_left);
    free(sorted_right);

    return sum;
}

static long part_02(const int *left, const int *right, int count) {
    long sum = 0;

    for (int i = 0; i < count; i++) {
        int occurrences = 0;

        for (int j = 0; j < count; j++) {
            if (right[j] == left[i]) {
                occurrences++;
            }
        }

        sum += (long)occurrences * left[i];
    }

    return sum;
}

int main(int argc, char **argv) {
    const char *path = argc > 1 ? argv[1] : "input.txt";

    int left[MAX_LINES];
    int right[MAX_LINES];

    int count = parse_input(path, left, right);

    printf("part 1: %ld\n", part_01(left, right, count));
    printf("part 2: %ld\n", part_02(left, right, count));

    return EXIT_SUCCESS;
}
