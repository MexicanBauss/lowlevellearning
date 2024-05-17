#include <stdio.h>
#include <stdlib.h>

typedef struct fraction {
    int numerator;
    int denominator;
} fraction_t ;

int gcd(int a, int b) 
{
    if (b==0) {
        return a;
    } else {
        return gcd(b, a % b);
    }
};

int lcm(int a, int b) 
{
    return abs(a * b) / gcd(a,b);
};

fraction_t add_fractions(fraction_t f1, fraction_t f2) {
    int denom = lcm(f1.denominator, f2.denominator);
    int numer = (denom / f1.denominator) * f1.numerator +
                (denom / f2.denominator) * f2.numerator;

    fraction_t result = {numer, denom};
    return result;
};

int main() {
    fraction_t f1 = {1,3};
    fraction_t f2 = {2,5};
    fraction_t f3 = add_fractions(f1,f2);
    printf("Result: %d/%d\n", f3.numerator, f3.denominator);

    return EXIT_SUCCESS;
}
