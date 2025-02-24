#include "mt19937-64.c"
#include <iostream>
#include <vector>
#include <cmath>
#include <omp.h>
using namespace std;

void counting_sort(vector<unsigned long long>& arr, int n, int exp, int b, vector<int>& count, vector<unsigned long long>& output, double& fill_time, double& count_time, double& prefix_time, double& output_time, double& copyback_time);
void radix_sort(vector<unsigned long long>& arr, int n, int b, vector<int>& count, vector<unsigned long long>& output);


int main (int argc, char *argv[]) {
  
  int n;
  cout << "N: ";
  cin >> n;

  int b;
  cout << "B: ";
  cin >> b;

  if (b < 0 || (b & (b - 1)) != 0) {
    cout << "Invalid B, please only give B that is a power of 2.";
    return 0;
  }

  cout << "Registered N = " << n << " and B = " << b << "\n";

  int before = omp_get_wtime();
  // Allocate vector
  vector<unsigned long long> nums(n);
  // Generate random 64bit numbersz
  for (int i = 0; i < n; i++) {
    nums[i] = genrand64_int64();
  }
  cout << "Time spent generating numbers: " << omp_get_wtime() - before << "\n";

  // Allocate counting vector
  vector<int> counting(pow(2.0, b));

  // Allocate output vector
  vector<unsigned long long> output(n);

  before = omp_get_wtime();

  radix_sort(nums, n, b, counting, output);

  cout << "Time spent sorting: " << omp_get_wtime() - before << "\n";

  // Verify array is sorted
  bool is_sorted = true;
  for (int i = 1; i < n; i++) {
    if (nums[i] < nums[i-1]) {
      is_sorted = false;
      break;
    }
  }
  cout << "Array is " << (is_sorted ? "sorted" : "not sorted") << "\n";

  cout << nums[0];

  return 0;
}

void radix_sort(vector<unsigned long long>& arr, int n, int b, vector<int>& count, vector<unsigned long long>& output) {
    int num_passes = (64 + b - 1) / b;
    
    double fill_time = 0, count_time = 0, prefix_time = 0, output_time = 0, copyback_time = 0;

    for (int pass = 0; pass < num_passes; pass++) {
        int exp = pass * b;
        counting_sort(arr, n, exp, b, count, output, fill_time, count_time, prefix_time, output_time, copyback_time);
    }

    
    cout << "Fill time: " << fill_time << "s\n"
         << "Count time: " << count_time << "s\n"
         << "Prefix time: " << prefix_time << "s\n"
         << "Output time: " << output_time << "s\n"
         << "Copyback time: " << copyback_time << "s\n";
}


void counting_sort(vector<unsigned long long>& arr, int n, int exp, int b, vector<int>& count, 
                  vector<unsigned long long>& output, double& fill_time, double& count_time, 
                  double& prefix_time, double& output_time, double& copyback_time) {
    double before;

    before = omp_get_wtime();
    fill(count.begin(), count.end(), 0);
    fill_time += omp_get_wtime() - before;
    
    before = omp_get_wtime();
    for (int i = 0; i < n; i++) {
        unsigned long long digit = (arr[i] >> exp) & ((1ULL << b) - 1);
        count[digit]++;
    }
    count_time += omp_get_wtime() - before;
    
    before = omp_get_wtime();
    for (int i = 1; i < count.size(); i++) {
        count[i] += count[i - 1];
    }
    prefix_time += omp_get_wtime() - before;
    
    before = omp_get_wtime();
    for (int i = n - 1; i >= 0; i--) {
        unsigned long long digit = (arr[i] >> exp) & ((1ULL << b) - 1);
        output[count[digit] - 1] = arr[i];
        count[digit]--;
    }
    output_time += omp_get_wtime() - before;
    
    before = omp_get_wtime();
    for (int i = 0; i < n; i++) {
        arr[i] = output[i];
    }
    copyback_time += omp_get_wtime() - before;
}
