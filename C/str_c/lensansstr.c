#include <stdio.h>
#include <stdlib.h>

int getLen(char *array) {
    int i = 0;
    while (array[i] != '\n') {
        i++;
    }
    return i;
}

int main () {
    char buffer[64];
    printf("Please introduce a string: ");
    fgets(buffer, 64, stdin);
    int len = getLen(buffer);
    printf("The length of the string is: %d\n", len);
    return EXIT_SUCCESS;
}

