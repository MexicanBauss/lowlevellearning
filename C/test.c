#include <stdio.h>
#include <stdlib.h>

long factorial(long n) {
    if (n<=0) {
        return 1;
    };

    return n * factorial(n-1);
};

int main() {
    long j = 0;
        while(j<21){
        long facto = factorial(j);
            printf("Factorial %ld!:\t%ld\n", j, facto);
        j++;
    }
    printf("\n");
    return 0; 
}

