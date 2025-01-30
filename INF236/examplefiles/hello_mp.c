#include <stdio.h>
#include <omp.h>

int main() {

int winner;
omp_set_num_threads(4);
#pragma omp parallel 
{
   // int ID = 0;
   int ID = omp_get_thread_num();
   printf("Hello (%d) ",ID);
   printf("world (%d)\n",ID);

   winner = ID;
}

printf("Done!\n");
printf("winner = %d\n",winner);

// omp_set_num_threads(2);
#pragma omp parallel 
{
   // int ID = 0;
   int ID = omp_get_thread_num();
   printf("Hello (%d) ",ID);
   printf("world (%d)\n",ID);

   winner = ID;
}

printf("Final winner = %d\n",winner);
}
