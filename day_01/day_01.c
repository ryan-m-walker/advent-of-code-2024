#include <stdio.h>
#include <stdlib.h>

typedef struct {
    long long *values;
    size_t length;
    size_t capacity;
} NumberList;

static void free_list(NumberList *list) {
    free(list->values);
    list->values = NULL;
    list->length = 0;
    list->capacity = 0;
}

static int append(NumberList *list, long long value) {
    if (list->length == list->capacity) {
        size_t new_capacity = list->capacity == 0 ? 16 : list->capacity * 2;
        long long *new_values = realloc(list->values, new_capacity * sizeof(*new_values));
        if (new_values == NULL) {
            return 0;
        }
        list->values = new_values;
        list->capacity = new_capacity;
    }

    list->values[list->length++] = value;
    return 1;
}

static int compare_numbers(const void *first, const void *second) {
    long long left = *(const long long *)first;
    long long right = *(const long long *)second;
    return (left > right) - (left < right);
}

static int read_input(const char *path, NumberList *left, NumberList *right) {
    FILE *input = fopen(path, "r");
    long long left_value;
    long long right_value;

    if (input == NULL) {
        perror(path);
        return 0;
    }

    while (fscanf(input, "%lld %lld", &left_value, &right_value) == 2) {
        if (!append(left, left_value) || !append(right, right_value)) {
            fprintf(stderr, "Unable to allocate memory for input.\n");
            fclose(input);
            return 0;
        }
    }

    if (!feof(input)) {
        fprintf(stderr, "Invalid input: expected two numbers per line.\n");
        fclose(input);
        return 0;
    }

    fclose(input);
    return 1;
}

static long long total_distance(const NumberList *left, const NumberList *right) {
    long long total = 0;
    size_t index;

    for (index = 0; index < left->length; index++) {
        long long difference = left->values[index] - right->values[index];
        total += difference < 0 ? -difference : difference;
    }

    return total;
}

static long long similarity_score(const NumberList *left, const NumberList *right) {
    long long total = 0;
    size_t left_index = 0;
    size_t right_index = 0;

    while (left_index < left->length && right_index < right->length) {
        if (left->values[left_index] < right->values[right_index]) {
            left_index++;
        } else if (left->values[left_index] > right->values[right_index]) {
            right_index++;
        } else {
            long long value = left->values[left_index];
            size_t occurrences = 0;

            while (right_index < right->length && right->values[right_index] == value) {
                occurrences++;
                right_index++;
            }

            while (left_index < left->length && left->values[left_index] == value) {
                total += value * (long long)occurrences;
                left_index++;
            }
        }
    }

    return total;
}

int main(int argc, char **argv) {
    NumberList left = {0};
    NumberList right = {0};
    long long part_one;
    long long part_two;

    if (argc != 2) {
        fprintf(stderr, "Usage: %s INPUT_FILE\n", argv[0]);
        return EXIT_FAILURE;
    }

    if (!read_input(argv[1], &left, &right)) {
        free_list(&left);
        free_list(&right);
        return EXIT_FAILURE;
    }

    qsort(left.values, left.length, sizeof(*left.values), compare_numbers);
    qsort(right.values, right.length, sizeof(*right.values), compare_numbers);
    part_one = total_distance(&left, &right);
    part_two = similarity_score(&left, &right);

    printf("Part 1: %lld\nPart 2: %lld\n", part_one, part_two);

    free_list(&left);
    free_list(&right);
    return EXIT_SUCCESS;
}
