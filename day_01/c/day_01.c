#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int *items;
    size_t len;
    size_t cap;
} IntList;

static void push(IntList *list, int value) {
    if (list->len == list->cap) {
        size_t cap = list->cap == 0 ? 16 : list->cap * 2;
        int *items = realloc(list->items, cap * sizeof(int));
        if (items == NULL) {
            fprintf(stderr, "Failed to allocate memory\n");
            exit(1);
        }
        list->items = items;
        list->cap = cap;
    }
    list->items[list->len++] = value;
}

static int compare(const void *a, const void *b) {
    int left = *(const int *)a;
    int right = *(const int *)b;
    return (left > right) - (left < right);
}

static void parse_input(const char *path, IntList *left, IntList *right) {
    FILE *file = fopen(path, "r");
    if (file == NULL) {
        fprintf(stderr, "Failed to read input file: %s\n", path);
        exit(1);
    }

    int l, r;
    while (fscanf(file, "%d %d", &l, &r) == 2) {
        push(left, l);
        push(right, r);
    }

    fclose(file);
}

static long part_01(const IntList *left, const IntList *right) {
    int *sorted_left = malloc(left->len * sizeof(int));
    int *sorted_right = malloc(right->len * sizeof(int));
    if (sorted_left == NULL || sorted_right == NULL) {
        fprintf(stderr, "Failed to allocate memory\n");
        exit(1);
    }

    for (size_t i = 0; i < left->len; i++) {
        sorted_left[i] = left->items[i];
        sorted_right[i] = right->items[i];
    }

    qsort(sorted_left, left->len, sizeof(int), compare);
    qsort(sorted_right, right->len, sizeof(int), compare);

    long sum = 0;
    for (size_t i = 0; i < left->len; i++) {
        sum += abs(sorted_right[i] - sorted_left[i]);
    }

    free(sorted_left);
    free(sorted_right);

    return sum;
}

static long part_02(const IntList *left, const IntList *right) {
    long sum = 0;

    for (size_t i = 0; i < left->len; i++) {
        long occurrences = 0;
        for (size_t j = 0; j < right->len; j++) {
            if (right->items[j] == left->items[i]) {
                occurrences++;
            }
        }
        sum += occurrences * left->items[i];
    }

    return sum;
}

int main(int argc, char **argv) {
    const char *path = argc > 1 ? argv[1] : "input.txt";

    IntList left = {0};
    IntList right = {0};

    parse_input(path, &left, &right);

    if (left.len != right.len) {
        fprintf(stderr, "Mismatched list lengths\n");
        return 1;
    }

    printf("part 1: %ld\n", part_01(&left, &right));
    printf("part 2: %ld\n", part_02(&left, &right));

    free(left.items);
    free(right.items);

    return 0;
}
