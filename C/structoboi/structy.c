#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct fraction {
    int numerator;
    int denominator;
}; 

struct student {
    char name[32];
    short class_year;
    int age; 
};

int main() {

    struct student s = { .name = "H.Sommers", .class_year = 2026, .age = 5};
    printf("Name: %s\nClass year: %d\nAge: %d\n", s.name, s.class_year, s.age); 
    return 0;

}
