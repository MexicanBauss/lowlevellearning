#include <stdio.h>
#include <stdlib.h>

int main() {
    // Define vars
    char dog_name[64];
    char dog_age_str[64];

    printf("What is the name of the dog?\n");
    fgets(dog_name, 64, stdin);

    printf("How old is the dog?\n");
    fgets(dog_age_str, 64, stdin);

    int dog_age = atoi(dog_age_str);
    int dog_age_in_years = dog_age*7;

    printf("%s is %d years old in dog years.\n", dog_name, dog_age_in_years);

    return EXIT_SUCCESS;
};