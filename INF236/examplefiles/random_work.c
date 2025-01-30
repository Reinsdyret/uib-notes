#include <stdio.h>
#include <stdlib.h>
#include <omp.h>

#define size 200000
#define nthreads 5

int main() {


  int i;
  int total = 0;
//  omp_set_num_threads(nthreads);

  double start = omp_get_wtime();

  #pragma omp parallel for schedule(runtime)
  for(i=0;i<size;i++) {
    int j;
    int sum=0;
    for(j=0;j<i;j++)
      sum = sum + j;
    total += sum;
  }

  double end = omp_get_wtime();

  printf("Time used %lf \n",end-start);
  printf("total = %d \n",total);
}
