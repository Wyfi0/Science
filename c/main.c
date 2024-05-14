#include <stdio.h>


int main() {
    double divisor = 1;
    double result = 0;

    for (int i = 0; i < 1000000000; i++) {

            double sub_result = 4 / divisor;

            if (i % 2 == 0) {
                result = result + sub_result;
            } 
            else {
                result = result - sub_result;
            }

            divisor += 2;
    }
    printf("%0.50f\n", result);
} 
