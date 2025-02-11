#include <iostream>
#include <time.h>
#include <stdlib.h>
#include <omp.h>
using namespace std;

void counting_sort(int arr[], int N) {
  double start, end;

  // Double loops
  for (int i = 0; i < N; i++) {
    int num = arr[i];

    for (int j = 0; j < N; j++) {
      if arr[j] < 
    }
  }

  // Find max elem
  int max_elem = arr[0];

  start = omp_get_wtime();
// #pragma omp parallel for reduction(max:max_elem) ITS SHITE
  for (int i = 0; i < N; i++) {
    if (arr[i] > max_elem) {
      max_elem = arr[i];
    }
  }
  end = omp_get_wtime();
  cout <<fixed<< "Time for finding max elem: " << end - start << "\n";

  int count_array[max_elem + 1] = {0};

  start = omp_get_wtime();
#pragma omp parallel for reduction(+:count_array[:max_elem+1]) 
  for(int i = 0; i < N; i++) {
    count_array[arr[i]] += 1;
  }
  end = omp_get_wtime();
  cout <<fixed<< "Time for counting: " << end - start << "\n";

  start = omp_get_wtime();
  for(int i = 1; i < max_elem + 1; i++) {
    count_array[i] = count_array[i-1] + count_array[i];
  }
  end = omp_get_wtime();
  cout << fixed << "Time for prefix sum count: " << end - start << "\n";

  start = omp_get_wtime();

  int output_array[N];

  for(int i = N-1; i > -1; i--) {
    output_array[count_array[arr[i]] - 1] = arr[i];
    count_array[arr[i]] = count_array[arr[i]] - 1;
  }

  end = omp_get_wtime();
  cout << "Time for making output arr: " << end - start << "\n";

  start = omp_get_wtime();
  for (int i = 0; i < N; i++) {
    arr[i] = output_array[i];
  }
  end = omp_get_wtime();
  cout << "Time for transferring output arr: " << end - start << "\n";
}

bool is_sorted(int arr[], int n) {
  for(int i = 0; i < n-1; i++) {
    if (arr[i] > arr[i + 1]) {
      return false;
    }
  }
  return true;
}

void init_arr(int arr[], int n, int max) {
  for(int i = 0; i < n; i++) {
    arr[i] = rand() % max;
  }
}

int main (int argc, char *argv[]) {
  int n = 10000;
  int test[n];

  init_arr(test, n, n);
  
  double before = omp_get_wtime();
  counting_sort(test, n);
  double after = omp_get_wtime();

  cout << "It took " << after - before << "\n";

  if (!is_sorted(test, n)) {
    cout << "Array not sorted \n";
  } else {
    cout << "Array is sorted \n";
  }

  return 0;
}
