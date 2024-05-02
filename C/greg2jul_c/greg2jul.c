#include <stdio.h>
#include <stdlib.h>

int main() {

    char year_str[64];
    char month_str[64];
    char day_str[64];
    char hour_str[64];
    char minute_str[64];
    char second_str[64];

    printf("Please introduce a year:\n");
    fgets(year_str, 64, stdin);

    printf("Please introduce a month:\n");
    fgets(month_str, 64, stdin);

    printf("Please introduce a day:\n");
    fgets(day_str, 64, stdin);

    printf("Please introduce an hour:\n");
    fgets(hour_str, 64, stdin);

    printf("Please introduce a minute:\n");
    fgets(minute_str, 64, stdin);

    printf("Please introduce a second:\n");
    fgets(second_str, 64, stdin);

    float year = atoi(year_str);
    float month = atoi(month_str);
    float day = atoi(day_str);
    float hour = atoi(hour_str);
    float minute = atoi(minute_str);
    float second = atoi(second_str);

    float JDN = (1461 * (year + 4800 + (month - 14)/12))/4 + (367 * (month - 2 - 12 * ((month - 14)/12)))/12 - (3 * ((year + 4900 + (month - 14)/12)/100))/4 + day - 32075;
    float JD = JDN + ((hour-12)/24) + minute/1440 + second/86400;

    printf("Julian Date: %f\n", JD);

}