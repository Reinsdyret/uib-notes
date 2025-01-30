#include <stdio.h>
#include <omp.h>

int main() {
  int n = 8;
  int arr[n];
  #pragma omp parallel
  {
   int ID = omp_get_thread_num();
//   int ID = 0;
   printf("Hello (%d) ",ID);
   printf("world (%d)\n",ID);
   arr[ID] = ID;
  }

  for(int i = 0; i < n; i++) {
    printf("%d", arr[i]);
  }
 
}
