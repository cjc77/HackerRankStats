#include <stdio.h>
#include <string.h>
#include <math.h>
#include <stdlib.h>

void readArray(int ** array, int * size);
void findQuartiles(int * array, int size);
void median(int * array, int lowerBound, int upperBound);

int compare(const void * a, const void * b);


int main()
{
    int * data = NULL;
    int size = 0;
    readArray(& data, & size);
    findQuartiles(data, size);
    free(data);
    return 0;
}



void readArray(int ** array, int * size)
{
    scanf("%d", size);
    * array = malloc(sizeof(int) * (*size));
    // Read
    for(int i = 0; i < *size; ++i) {
        scanf("%d", (*array + i));
    }
    // Sort
    qsort(*array, *size, sizeof(int), compare);
}


// Find quartiles
void findQuartiles(int * array, int size)
{
    // Q1
    median(array, 0, size / 2 - 1);
    // Q2
    median(array, 0, size - 1);
    // Q3
    if(size % 2 == 0)
        median(array, size / 2, size - 1);
    else
        median(array, size / 2 + 1, size - 1);

}

// Prints the quartile
void median(int * array, int low, int high)
{
    int idx1 = 0; int idx2 = 0; float med = 0;
    // Odd num of indices
    if( (high - low) % 2 == 0 ) {
        idx1 = (low + high) / 2;
        med = *(array + idx1);
        printf("%.0f\n", med);
    }
    // Even num of indices
    else {
        idx1 = (low + high) / 2; idx2 = idx1 + 1;
        med = ( (float)*(array + idx1) + (float)*(array + idx2) ) / 2;
        printf("%.0f\n", med);
    }
}


// Helper function for sorting
int compare(const void * a, const void * b)
{
    if( *(int*)a == *(int*)b ) return 0;
    return *(int*)a < *(int*)b ? -1 : 1;
}
