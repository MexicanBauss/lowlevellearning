#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_STUDENTS 100
#define MAX_NAME_LENGTH 32
#define MAX_INPUT_LENGTH 64

struct student {
    char name[MAX_NAME_LENGTH];
    short class_year;
    char age;
};

int main() {
    char buffer[MAX_INPUT_LENGTH];
    struct student stdnts[MAX_STUDENTS];
    int num_students = 0;
    float avg_age = 0;

    printf("Enter student information (name, class year, age), separate by comma. Type 'DONE' to finish:\n");
    while (fgets(buffer, MAX_INPUT_LENGTH, stdin) != NULL && strncmp(buffer, "DONE", 4) != 0 && num_students < MAX_STUDENTS) {
        char *token = strtok(buffer, ",");

        strcpy(stdnts[num_students].name, token);
        token = strtok(NULL, ",");
        stdnts[num_students].class_year = atoi(token);
        token = strtok(NULL, ",");
        stdnts[num_students].age = atoi(token);
        
        avg_age += stdnts[num_students].age;
        
        num_students++;

        printf("Name: %s\nClass Year: %d\nAge: %d\n", stdnts[num_students-1].name, stdnts[num_students-1].class_year, stdnts[num_students-1].age);
    };

    if (num_students > 0) {
        avg_age /= num_students;
        printf("Average age of students: %.2f\n", avg_age);
    }

    char name[MAX_NAME_LENGTH];

    printf("Enter a student name to search (type 'DONE' to exit): ");
    while (fgets(name, MAX_NAME_LENGTH, stdin) != NULL && strncmp(name, "DONE", 4) != 0) {
        int matching_students = 0;
        for (int i = 0; i < num_students; i++) {
            if (strcasecmp(stdnts[i].name, name) == 0) {
                printf("Student: %s; Class year: %d; Age: %d\n", stdnts[i].name, stdnts[i].class_year, stdnts[i].age);
                matching_students++;
            }
        };
        if (matching_students < 1) {
            printf("No students match the name: %s\n", name);
        }
        printf("Enter a student name to search (type 'DONE' to exit): ");
    };

    return 0;
}
