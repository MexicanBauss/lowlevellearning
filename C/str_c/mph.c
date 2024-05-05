#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
    // Define buffers
    char miles_str[64];
    char time_str[64];
    char hour_str[32];
    char minute_str[32];
    char second_str[32];

    // Get input
    printf("How many miles: ");
    fgets(miles_str, 64, stdin);
    printf("How much time (HH:MM:SS): ");
    fgets(time_str, 64, stdin);

    // Get substrings
    strncpy(hour_str, &time_str[0], 2);
    strncpy(minute_str, &time_str[3], 2);
    strncpy(second_str, &time_str[7], 2);

    // Transform to floats
    float miles = atof(miles_str);
    float hour = atof(hour_str);
    float minute = atof(minute_str);
    float second = atof(second_str);
    float total_time = hour*60 + minute+ second/60;

    // MPH
    float mph = miles/total_time;
    printf("Pace: %f miles per minute\n", mph);

    return 0;
}
