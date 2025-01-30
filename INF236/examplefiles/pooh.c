#include "omp.h"
#include "stdio.h"

int main() {

  int A[40];

omp_set_num_threads(40);
#pragma omp parallel
  {
   int id = omp_get_thread_num();

   A[id] = id;
  }

  int i;
  for(i=0;i<40;i++)
    printf("%d \n",A[i]);

}
