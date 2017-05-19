#include <stdio.h>
#include <stdlib.h>
#include <math.h>


int getInput(int * size, int ** array);
float mean(int size, int * array);

int main()
{
    int * nums = NULL;
    int size, i;
    float dataMean;
    getInput(&size, &nums);
    dataMean = mean(size, nums);
    for(i = 0; i < size; i++) {
        printf("%d\n", nums[i]);
    }
    printf("Mean: %.1f\n", dataMean);
    return 0;
}



float mean(int size, int * array)
{
    int i;
    float sum = 0;
    for(i = 0; i < size; i++) {
        sum += array[i]; 
    }
    return sum / size;
}



int getInput(int * size, int ** array)
{
    int i;

    // Get the input of size and then read in the array
    if(scanf("%d", size)) {
        printf("\nReading...\n");
        *array = malloc(sizeof(int) * (*size)); 

        for(i = 0; i < *size; i++) {
            scanf("%d", (*array + i));
        }
        return 1;
    }
    return 0;
}
