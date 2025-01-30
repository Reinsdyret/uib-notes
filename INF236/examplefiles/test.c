/**********************************************************************
 * Example: Loop parallelism in OpenMP/C
 *
 * Insert directives to parallelize the main for loop. 
 * Experiement with different scheduling methods to see 
 * which gives best speedup.
 *
 **********************************************************************/

#include <omp.h>
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
  int i,n=20000000;
  int j;
  double *A,*B;
  double timer;
 
  A = (double *)malloc(n*sizeof(double));
  B = (double *)malloc(n*sizeof(double));

  timer=omp_get_wtime();
  double sum = 0.0;
  omp_set_num_threads(4);
#pragma omp parallel
#pragma omp for 
  for (i=0;i<n;i++){
    B[i] += A[i];
  }

  timer=omp_get_wtime()-timer;
  printf("Time: %f\n",timer);

  return 0;
}
