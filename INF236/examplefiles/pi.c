#include <stdio.h>
#include <omp.h>

#define NUM_THREADS 4

static long num_steps = 10000000; 
double step;

void main ()
{
  int i, nthreads; 
  double pi, sum[NUM_THREADS]; 

  step = 1.0/(double) num_steps; 
  omp_set_num_threads(NUM_THREADS);

  double start = omp_get_wtime();
#pragma omp parallel
  {
    int i, id,nthrds;
    double x;
 
    id = omp_get_thread_num(); 
    nthrds = omp_get_num_threads(); 
    if (id == 0) 
      nthreads = nthrds;
    sum[id] = 0.0;
    for (i=id, sum[id]=0.0;i< num_steps; i=i+nthrds) { // increasing i by nthrds
      x = (i+0.5)*step;
      sum[id] += 4.0/(1.0+x*x);
    }
  }

  double start1 = omp_get_wtime();
// Sequential sum
  for(i=0, pi=0.0;i<nthreads;i++)
    pi += sum[i] * step;

  double end = omp_get_wtime();

  printf("Pi = %.10lf \n",pi);
  printf("Time = %lf \n",end-start);
  printf("Time for summing = %lf \n", start1 - end);
}
