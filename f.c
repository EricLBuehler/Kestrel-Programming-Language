#include <stdio.h>
#include <string.h>
#include <math.h>

int main(int argc, char** argv) {
    unsigned char f = 1;
    char arr[20];

    sprintf(arr, "%u", f);

    printf(arr);
}