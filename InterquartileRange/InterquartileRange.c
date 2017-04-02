#include <stdio.h>
#include <stdlib.h>


void readArrays(int ** array1, int ** array2, int * size, int * fullSize);
void makeMaster(int ** master, int * nums, int * freq, int size, int newSize);
int compare(const void * a, const void * b);
float median(int * array, int low, int high);
float quartile(int * array, int size, int qNum);




int main()
{
    int * data = NULL;
    int * freq = NULL;
    int * master = NULL;
    int size = 0; int fullSize = 0;
    float q1 = 0.0; float q2 = 0.0; float q3 = 0.0;
    float qRange = 0;

    readArrays(& data, & freq, & size, & fullSize);
    makeMaster(& master, data, freq, size, fullSize);
    q1 = quartile(master, fullSize, 1);
    q2 = quartile(master, fullSize, 2);
    q3 = quartile(master, fullSize, 3);
    qRange = q3 - q1;
    printf("%.1f\n", qRange);

    return 0;
}


void readArrays(int ** array1, int ** array2, int * size, int * fullSize)
{
    scanf("%d", size);
    *array1 = malloc(sizeof(int) * (*size));
    *array2 = malloc(sizeof(int) * (*size));
    for(int i = 0; i < *size; ++i) {
        scanf("%d", (*array1 + i));
    }
    for(int i = 0; i < *size; ++i) {
        scanf("%d", (*array2 + i));
    }
    for(int i = 0; i < *size; ++i) {
        *fullSize += *(*array2 + i);
    }
}


void makeMaster(int ** master, int * nums, int * freq, int size, int newSize)
{
    int idx = 0;
    * master = malloc(sizeof(int) * newSize);
    for(int i = 0; i < size; ++i) {
        for(int j = 0; j < *(freq + i); ++j) {
            *(*master + idx) = *(nums + i);
            ++idx;
        }
    }
    qsort(*master, newSize, sizeof(int), compare);
}


float median(int * array, int low, int high)
{
    int idx1 = 0; int idx2 = 0; float med = 0;

    // Odd number of indices
    if( (high - low) % 2 == 0 ) {
        idx1 = (high + low) / 2;
        med = *(array + idx1);
    }
    // Even number of indices
    else {
        idx1 = (high + low) / 2;
        idx2 = idx1 + 1;
        med = ( (float)*(array + idx1) + (float)*(array + idx2) ) / 2;
    }
    return med;
}


float quartile(int * array, int size, int qNum)
{
    if(qNum == 1)
        return median(array, 0, size / 2 - 1);
    else if(qNum == 2)
        return median(array, 0, size - 1);
    else {
        if(size % 2 == 0)
            return median(array, size / 2, size - 1);
        else
            return median(array, size / 2 + 1, size - 1);
    }
}


int compare(const void * a, const void * b)
{
    if( *(int*)a == *(int*)b )
        return 0;
    return *(int*)a < *(int*)b ? -1 : 1;
}
