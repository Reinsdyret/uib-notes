#include <stdio.h>
#include <omp.h>
#include <cstdlib>
#include <iostream>
using namespace std;


int main (int argc, char *argv[]) {
  int n=10000;
  int **A,**B;
  double timer;


  A = (int**)malloc(n*sizeof(int*));
  B = (int**)malloc(n*sizeof(int*));

  for (int i = 0; i < n; i++) {
    A[i] = (int*)malloc(n*sizeof(int));
    B[i] = (int*)malloc(n*sizeof(int));
    for (int j = 0; j < n; j++) {
      A[i][j] = rand() % 100;
      B[i][j] = rand() % 100;
    }
  }

  timer=omp_get_wtime();

  #pragma omp parallel for num_threads(5)
  for (int i = 0; i < n; i++) {
    for (int j = 0;j < n; j++) {
      A[i][j] = max(A[i][j],B[i][j]);
    }
  }
 
  timer=omp_get_wtime()-timer;
  printf("Time: %f\n",timer);

  
  /*

  for (int i = 0; i < n; i++) {
    for (int j = 0;j < n; j++) {
      cout << A[i][j] << " ";
    }
    cout << "\n";
  } 
  cout << "\n"; 
  for (int i = 0; i < n; i++) {
    for (int j = 0;j < n; j++) {
      cout << B[i][j] << " ";
    }
    cout << "\n";
  } 
  */

  
  return 0;
}
