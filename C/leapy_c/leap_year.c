#include <stdio.h>
#include <stdlib.h>

int main() 
{
    char year_str[64];
    printf("Please introduce a year:\n");
    fgets(year_str, 64, stdin);
    int year = atoi(year_str);

    if ((year % 4 == 0 && year % 100 != 0) || year % 400 == 0) {
        printf("Year %d is a leap year.\n", year);
        return EXIT_SUCCESS;
    }
    // if not leap year
    printf("Year %d is not a leap year.\n", year);
    return EXIT_SUCCESS;
}