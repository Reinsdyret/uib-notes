#include <stdio.h>
#include <omp.h>

#define NUM_THREADS 4

static long num_steps = 10000000; 
double step;

void main ()
{
  int i, nthreads; 
  double pi= 0.0;
  FILE *fp;

  fp = fopen("output","w");
  if (fp == NULL)
    printf("Bad file day!");

  step = 1.0/(double) num_steps; 
  omp_set_num_threads(NUM_THREADS);

  double start = omp_get_wtime();
#pragma omp parallel
  {
    int i, id,nthrds;
    double x,sum = 0.0;
    id = omp_get_thread_num(); 
    nthrds = omp_get_num_threads(); 
    if (id == 0) 
      nthreads = nthrds;

    for (i=id, sum=0.0;i< num_steps; i=i+nthrds) { 
      x = (i+0.5)*step;
      sum += 4.0/(1.0+x*x);
    }
    #pragma omp critical
    pi += sum * step;
  }
  double end = omp_get_wtime();


  printf("Pi = %.10lf \n",pi);
  printf("Time: %lf \n",end-start);
}
