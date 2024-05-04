#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
    char buffer[64];
    printf("Please provide a string: ");
    fgets(buffer, 64, stdin);
    int i, j = 0;
    char reverse[64];
    for (i = strlen(buffer) - 2;i>=0;i--) {
        reverse[j++] = buffer[i];
    }
    printf("Old string: %s\n", buffer);
    printf("New string: %s\n", reverse);
    return EXIT_SUCCESS;
}