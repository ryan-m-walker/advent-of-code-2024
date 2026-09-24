#include <stdio.h>
#include <stdlib.h>

typedef struct {
    long *left;
    long *right;
    size_t length;
    size_t capacity;
} Lists;

static void free_lists(Lists *lists)
{
    free(lists->left);
    free(lists->right);
    lists->left = NULL;
    lists->right = NULL;
    lists->length = 0;
    lists->capacity = 0;
}

static int append_pair(Lists *lists, long left, long right)
{
    if (lists->length == lists->capacity) {
        size_t new_capacity = lists->capacity == 0 ? 16 : lists->capacity * 2;
        long *new_left = realloc(lists->left, new_capacity * sizeof(*new_left));
        long *new_right;

        if (new_left == NULL) {
            return 0;
        }

        new_right = realloc(lists->right, new_capacity * sizeof(*new_right));
        if (new_right == NULL) {
            free(new_left);
            lists->left = NULL;
            return 0;
        }

        lists->left = new_left;
        lists->right = new_right;
        lists->capacity = new_capacity;
    }

    lists->left[lists->length] = left;
    lists->right[lists->length] = right;
    lists->length++;
    return 1;
}

static int compare_longs(const void *first, const void *second)
{
    const long left = *(const long *)first;
    const long right = *(const long *)second;

    return (left > right) - (left < right);
}

static long absolute_difference(long left, long right)
{
    return left > right ? left - right : right - left;
}

static long solve_distance(const Lists *lists)
{
    long *left = malloc(lists->length * sizeof(*left));
    long *right = malloc(lists->length * sizeof(*right));
    long result = 0;
    size_t index;

    if (lists->length == 0) {
        free(left);
        free(right);
        return 0;
    }

    if (left == NULL || right == NULL) {
        free(left);
        free(right);
        return 0;
    }

    for (index = 0; index < lists->length; index++) {
        left[index] = lists->left[index];
        right[index] = lists->right[index];
    }

    qsort(left, lists->length, sizeof(*left), compare_longs);
    qsort(right, lists->length, sizeof(*right), compare_longs);

    for (index = 0; index < lists->length; index++) {
        result += absolute_difference(left[index], right[index]);
    }

    free(left);
    free(right);
    return result;
}

static long solve_similarity(const Lists *lists)
{
    long result = 0;
    size_t left_index;

    for (left_index = 0; left_index < lists->length; left_index++) {
        long occurrences = 0;
        size_t right_index;

        for (right_index = 0; right_index < lists->length; right_index++) {
            if (lists->left[left_index] == lists->right[right_index]) {
                occurrences++;
            }
        }

        result += lists->left[left_index] * occurrences;
    }

    return result;
}

int main(int argc, char **argv)
{
    FILE *input = stdin;
    Lists lists = {0};
    long left;
    long right;
    int status = EXIT_SUCCESS;

    if (argc > 2) {
        fprintf(stderr, "Usage: %s [input-file]\n", argv[0]);
        return EXIT_FAILURE;
    }

    if (argc == 2) {
        input = fopen(argv[1], "r");
        if (input == NULL) {
            perror(argv[1]);
            return EXIT_FAILURE;
        }
    }

    while (fscanf(input, "%ld %ld", &left, &right) == 2) {
        if (!append_pair(&lists, left, right)) {
            fprintf(stderr, "Unable to allocate memory for input\n");
            status = EXIT_FAILURE;
            goto cleanup;
        }
    }

    if (!feof(input)) {
        fprintf(stderr, "Invalid input: expected pairs of integers\n");
        status = EXIT_FAILURE;
        goto cleanup;
    }

    printf("Part 1: %ld\nPart 2: %ld\n", solve_distance(&lists), solve_similarity(&lists));

cleanup:
    if (input != stdin) {
        fclose(input);
    }
    free_lists(&lists);
    return status;
}
