#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct student {
    char name[32];
    short class_year;
    char age;
};

int main() {
    char buffer[64];
    struct student stdnts[100];
    int num_students = 0;
    float avg_age;
    // printf("Please introduce the name of a student, its class year, and the age.");
    while (fgets(buffer, 64, stdin) != NULL) {
        char *token = strtok(buffer, ",");

        strcpy(stdnts[num_students].name, token);
        token = strtok(NULL, ",");
        stdnts[num_students].class_year = atoi(token);
        token = strtok(NULL, ",");
        stdnts[num_students].age = atoi(token);
        
        avg_age += atof(token);
        
        num_students++;

        printf("Name: %s\nClass Year: %d\nAge: %d\n", stdnts[num_students-1].name,stdnts[num_students-1].class_year, stdnts[num_students-1].age);

    };

    char name[32];

    printf("Look for a name: ");
    while (fgets(name, 32, stdin) != NULL && strncmp(name, "DONE", 4) != 0) {
        int matching_students = 0;
        for (int i = 0; i < num_students; i++) {
            if (strcasecmp(stdnts[i].name, name) == 0) {
                printf("Student: %s; Class year: %d; Year: %d\n", stdnts[i].name, stdnts[i].class_year, stdnts[i].age);
                matching_students++;
            }
        };
        if (matching_students < 1) {
            printf("No students match the name: %s\n", name);
        }
        printf("Look for a name: ");
    };

    return 0;
}