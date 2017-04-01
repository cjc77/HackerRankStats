#include <stdio.h>
#include <string.h>
#include <math.h>
#include <stdlib.h>


float mean(int * array, int size);
float median(int * array, int size);
int mode (int * array, int size);
int compare(const void * a, const void * b);



int main()
{
    int size = 0;
    float avg = 0.0;
    float med = 0.0;
    int mod = 0;

    scanf("%d", &size);
    int * array = (int *)malloc(sizeof(int) * size);
    // Read it in
    for(int i = 0; i < size; ++i) {
        scanf("%d", (array + i));
    }
    // Sort it
    qsort(array, size, sizeof(int), compare);
    // Print it
//    for(int i = 0; i < size; ++i) {
//        printf("%d\n", *(array + i));
//    }
    avg = mean(array, size);
    med = median(array, size);
    mod = mode(array, size);
    printf("%.1f\n", avg);
    printf("%.1f\n", med);
    printf("%d\n", mod);
    free(array);
    return 0;
}



float mean(int * array, int size)
{
    int sum = 0;
    for(int i = 0; i < size; ++i) {
        sum += *(array + i);
    }
    return (float)sum/(float)size;
}



float median (int * array, int size)
{
    int idx1 = 0;
    int idx2 = -1;
    float med = 0;
    // if even
    if (size % 2 == 0) {
        idx1 = size / 2 - 1;
        idx2 = size / 2;
    }
    else {
        idx1 = size / 2;
    }
    // if size was even
    if (idx2 > 0) {
        med = (float)*(array + idx1) + (float)*(array + idx2);
        return med / 2;
    }
    med = (float)*(array + idx1);
    return med;
}


int mode (int * array, int size)
{
    int curMode = 0; int count = 0; int maxCount = 0;

    for(int i = 0; i < size; ++i) {
        count = 0;

        // Find num occurances of array[i]
        for (int j = 0; j < size; ++j) {
            if (*(array + j) == *(array + i))
                ++count;
        }

        // See if this item appears more times than a previous item
        if (count > maxCount) {
            maxCount = count;
            curMode = *(array + i);
        }
    }
    return curMode;
}


int compare(const void * a, const void * b)
{
    if( *(int*)a == *(int*)b ) return 0;
    return *(int*)a < *(int*)b ? -1 : 1;
}
