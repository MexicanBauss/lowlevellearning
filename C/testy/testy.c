#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_INDEX 100
#define MAX_INPUT_LENGTH 64

void printArray(int *array, int size) {
    printf("Array: [ ");
    for (int i = 0; i < size; i++) {
        if (i < size - 1) {
            printf("%d, ", array[i]);
        } else {
            printf("%d ", array[i]);
        }
    }
    printf("]\n");
}

int main() {
    int scores[MAX_INDEX];
    memset(scores, 0, MAX_INDEX * sizeof(int));

    int index;
    char input[MAX_INPUT_LENGTH];
    int choice;

    printf("What do you want to do?\n1. Read Index\n2. Write to index\n3. Clear Index\n4. See array\n");
    scanf("%d", &choice);

    if (choice >= 1 && choice <= 4) {
        switch (choice) {
            case 1:
                printf("Enter an index between 0 and %d: ", MAX_INDEX - 1);
                scanf("%d", &index);
                if (index >= 0 && index < MAX_INDEX) {
                    printf("Value at index %d: %d\n", index, scores[index]);
                } else {
                    printf("Index out of bounds!\n");
                }
                break;
            case 2:
                printf("Enter an index between 0 and %d: ", MAX_INDEX - 1);
                scanf("%d", &index);
                if (index >= 0 && index < MAX_INDEX) {
                    printf("Enter a new value: ");
                    scanf("%s", input);
                    scores[index] = atoi(input); // Convert input string to integer
                } else {
                    printf("Index out of bounds!\n");
                }
                break;
            case 3:
                printf("Enter an index between 0 and %d: ", MAX_INDEX - 1);
                scanf("%d", &index);
                if (index >= 0 && index < MAX_INDEX) {
                    scores[index] = 0; // Clearing the value at the index
                    printf("Index %d cleared.\n", index);
                } else {
                    printf("Index out of bounds!\n");
                }
                break;
            case 4:
                printArray(scores, MAX_INDEX);
                break;
        }
    } else {
        printf("Invalid choice.\n");
    }

    return EXIT_SUCCESS;
}
