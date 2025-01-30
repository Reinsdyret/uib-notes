#include <stdio.h>
#include <omp.h>


static long num_steps = 10000000; 
double step;

void main ()
{
  int i; 

  step = 1.0/(double) num_steps; 

  double x,sum=0.0;
 
  double start = omp_get_wtime();

  for (i=0;i< num_steps; i++) {
      x = (i+0.5)*step;
      sum += 4.0/(1.0+x*x);
  }

  double pi = step*sum;
  double end = omp_get_wtime();

  printf("Pi = %.10lf \n",pi);
  printf("Time = %lf \n",end-start);
}
