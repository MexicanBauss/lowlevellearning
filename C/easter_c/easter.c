#include <stdio.h>
#include <stdlib.h>

/*
Write a program that asks for a year value and computes and prints the month and day of Easter in
that year. The Wikipedia page on Computus provides more than one algorithm for doing so. Try
using the "Anonymous Gregorian algorithm" or the "Gauss algorithm", which is a personal favorite
*/

int main() 
{
    char year_str[64];
    printf("Please introduce a year:\n");
    fgets(year_str, 64, stdin);
    int year = atoi(year_str);

    // Variables
    int a = year % 19;
    int b = year / 100;
    int c = year % 100;
    int d = b/4;
    int e = b % 4;
    int f = (b + 8)/25;
    int g = (b-f+1)/3;
    int h = (19*a+b-d-g+15) % 30;
    int i = c/4;
    int k = c % 4;
    int l = (32 + 2*e + 2*i - h - k) % 7;
    int m = (a + 11*h + 22*l)/451;
    int n = (h + l - 7*m + 114)/31;
    int o = (h + l - 7*m + 114) % 31;

    printf("Easter Date:\t%d/%d/%d\n", o+1, n, year);

    return EXIT_SUCCESS;

}