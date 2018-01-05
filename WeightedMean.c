#include <stdio.h>
#include <string.h>
#include <math.h>
#include <stdlib.h>


void readArrays(int ** numbers, int ** weights, int * size);
float weightedMean(int * numbers, int * weights, int size);


int main()
{
    int size = 0;
    int * numbers = NULL;
    int * weights = NULL;
    float answer = 0;

    readArrays(& numbers, & weights, & size);
    answer = weightedMean(numbers, weights, size);
    printf("%.1f\n", answer);
    free(numbers);
    free(weights);
    return 0;
}


void readArrays(int ** numbers, int ** weights, int * size)
{
    scanf("%d", size);
    // Build Empty arrays
    *numbers = malloc(sizeof(int) * (*size));
    *weights = malloc(sizeof(int) * (*size));
    // Read in numbers
    for(int i = 0; i < *size; ++i) {
        scanf("%d", (*numbers + i));
    }
    // Read in weights
    for(int i = 0; i < *size; ++i) {
        scanf("%d", (*weights + i));
    }
}


float weightedMean(int * numbers, int * weights, int size)
{
    float totalVals = 0;
    int totalSize = 0;
    for(int i = 0; i < size; ++i) {
        // add product of numbers[i] * weights[i] to totalVals
        totalVals += *(numbers + i) * *(weights + i);
        // add weights[i] to totalSize
        totalSize += *(weights + i);
    }
    return totalVals / totalSize;
}
