#include <stdio.h>
#include <omp.h>

#define NUM_THREADS 10

static long num_steps = 10000000; 
double step;
FILE *fp;

void main ()
{
  int i, nthreads; 
  double pi, sum=0.0; 
  fp = fopen("result","w");


  step = 1.0/(double) num_steps; 
//  omp_set_num_threads(NUM_THREADS);

  double start = omp_get_wtime();
  double x;

#pragma omp parallel for private(x) reduction(+:sum)
  for (i=0;i< num_steps; i++) { 
    x = (i+0.5)*step;
    sum += 4.0/(1.0+x*x);
  }

  pi += sum * step;

  double end = omp_get_wtime();


  fprintf(fp,"Pi = %.10lf \n",pi);
  printf("%lf \n",end-start);
}
