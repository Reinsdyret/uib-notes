#include <stdio.h>
#include <omp.h>


static long num_steps = 10000000; 
double step;

void main ()
{
  int threads = 4;
  omp_set_num_threads(threads);
  int sums[threads];
  int i; 

  step = 1.0/(double) num_steps; 

  #pragma omp parallel 
  {
  double x,sum=0.0;
 
  double start = omp_get_wtime();

  int ID = omp_get_thread_num();
  x = (ID + 0.5) * step;
  for (i=0;i< num_steps; i++) {
      x = (i+0.5)*step;
      sum += 4.0/(1.0+x*x);
  }

  }
  

  double pi = step*sum;
  double end = omp_get_wtime();

  printf("Pi = %.10lf \n",pi);
  printf("Time = %lf \n",end-start);
}
