#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[])
{
  int *arr = malloc(80000);
  arr[1000] = 50;
}
