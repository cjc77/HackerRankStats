#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#define SQUARE 2

struct data {
    int * numbers;
    float * deviations;
    int size;
    float dataMean;
    float dataDev;
};



int getInput(struct data * someData);
void mean(struct data * someData);
void stdDev(struct data * someData);



int main()
{
    struct data myData;

    if(getInput(&myData)) {
        mean(&myData);
        stdDev(&myData);
    }

    printf("%.1f\n", myData.dataDev);
    free(myData.numbers); free(myData.deviations);

    return 0;
}



void stdDev(struct data * someData)
{
    int i;
    int size = someData->size;
    float devSum = 0.0;

    for(i = 0; i < size; i++) {
        someData->deviations[i] = pow(((someData->numbers[i]) - someData->dataMean), SQUARE);
        devSum += someData->deviations[i];
    }
    someData->dataDev = sqrt(devSum / size);
}



void mean(struct data * someData)
{
    int i;
    int size = someData->size;
    float sum = 0.0;
    for(i = 0; i < size; i++) {
        sum += someData->numbers[i];
    }
    someData->dataMean = sum / size;
}



int getInput(struct data * someData)
{
    int i;

    // get size and "numbers" from user
    if(scanf("%d", &(someData->size))) {
        someData->numbers = malloc(sizeof(int) * someData->size);
        someData->deviations = malloc(sizeof(float) * someData->size);
        for(i = 0; i < someData->size; i++) {
            scanf("%d", &(someData->numbers[i]));
        }
        return someData->size;
    }
    return 0;
}
