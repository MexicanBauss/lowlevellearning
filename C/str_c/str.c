#include <stdio.h>
#include <stdlib.h>

int main() {
    int max = 10;
    int array[10] = { 1, 1, 2, 3, 5, 8, 13, 21, 34, 55 };

    for (int i = 0; i < max; i++ ) {
        printf("Array index %d contains %d\n", i, array[i]);
    }
    return EXIT_SUCCESS;
}

