#include <stdio.h>
#include <stdlib.h>

int main () {
    char buffer[64];
    printf("Please introduce a string: ");
    fgets(buffer, 64, stdin);

    char stripped_buffer[64]; // Buffer to store the stripped string
    int i = 0, j = 0;
    while (buffer[i] != '\0') {
        if (buffer[i] != ' ') {
            stripped_buffer[j++] = buffer[i];
        }
        i++;
    }
    stripped_buffer[j] = '\0'; // Null-terminate the stripped string

    printf("Your stripped string: %s\n", stripped_buffer);
    return EXIT_SUCCESS;
}

