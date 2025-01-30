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

double work(double c){
  double d=0.0;

  while (d<c){
    d=d+1.0;
  }
  return d;
}    

int main(int argc, char *argv[]) {
  int i,n=100000;
  double *A,*B;
  double timer;
 
  A = (double *)malloc(n*sizeof(double));
  B = (double *)malloc(n*sizeof(double));
  for (i=0;i<n;i++){
    A[i]=(double)i;
  }

  timer=omp_get_wtime();

#pragma omp parallel for num_threads(4) schedule(runtime)
  for (i=n-1;i>=0;i--){
    B[i]=work(A[i]);
  }
/*
#pragma omp parallel for num_threads(4) schedule(static)
#pragma omp parallel for num_threads(4) schedule(dynamic)
#pragma omp parallel for num_threads(4) schedule(guided)
#pragma omp parallel for num_threads(4) schedule(runtime)
*/

  timer=omp_get_wtime()-timer;
  printf("Time: %f\n",timer);

  return 0;
}
