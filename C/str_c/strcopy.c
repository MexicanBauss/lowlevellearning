#include <stdio.h>
#include <stdlib.h>


int main() {
    char buffer[64];
    printf("Please provide a string: ");
    fgets(buffer, 64, stdin);
    int i = 0;
    char new_string[64];
    do {
        new_string[i] = buffer[i];
        i++;
    } while (buffer[i] != '\0');
    printf("Old string: %s", buffer);
    printf("New string: %s\n", new_string);
    return EXIT_SUCCESS;
}