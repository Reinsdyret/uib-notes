
#include "mt19937-64.c"
#include <iostream>
#include <vector>
#include <cmath>
#include <omp.h>
using namespace std;

void counting_sort(vector<unsigned long long>& arr, int n, int exp, int b, vector<vector<int>>& count, vector<vector<int>>& prefix_arr, vector<unsigned long long>& output, double& fill_time, double& count_time, double& prefix_time, double& output_time, double& copyback_time);
void radix_sort(vector<unsigned long long>& arr, int n, int b, vector<vector<int>>& count, vector<vector<int>>& prefix_arr, vector<unsigned long long>& output);

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

  int threads;
#pragma omp parallel
{
  threads = omp_get_num_threads();
}
  // Allocate counting vector
  vector<vector<int>> counting(threads, vector<int>(pow(2.0,b)));

  // Allocate prefix vector
  vector<vector<int>> prefix_arr(threads, vector<int>(pow(2.0,b)));


  // Allocate output vector
  vector<unsigned long long> output(n);

  before = omp_get_wtime();

  radix_sort(nums, n, b, counting, prefix_arr, output);

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

void radix_sort(vector<unsigned long long>& arr, int n, int b, vector<vector<int>>& count, vector<vector<int>>& prefix_arr, vector<unsigned long long>& output) {
    int num_passes = (64 + b - 1) / b;

    double fill_time = 0, count_time = 0, prefix_time = 0, output_time = 0, copyback_time = 0;

    for (int pass = 0; pass < num_passes; pass++) {
        int exp = pass * b;
        counting_sort(arr, n, exp, b, count, prefix_arr, output, fill_time, count_time, prefix_time, output_time, copyback_time);
    }

    cout << "Fill time: " << fill_time << "s\n"
         << "Count time: " << count_time << "s\n"
         << "Prefix time: " << prefix_time << "s\n"
         << "Output time: " << output_time << "s\n"
         << "Copyback time: " << copyback_time << "s\n";
}

void counting_sort(vector<unsigned long long>& arr, int n, int exp, int b, vector<vector<int>>& count, vector<vector<int>>& prefix_arr, vector<unsigned long long>& output, double& fill_time, double& count_time, double& prefix_time, double& output_time, double& copyback_time) {
#pragma omp parallel
{
    int thread_id = omp_get_thread_num();
    double before;
    int num_threads = omp_get_num_threads();
    int radix = 1 << b;  // 2^b
    
#pragma omp single
{
    before = omp_get_wtime();
    for (auto& vec : count) {
        fill(vec.begin(), vec.end(), 0);
    }
    for (auto& vec : prefix_arr) {
        fill(vec.begin(), vec.end(), 0);
    }
    fill_time += omp_get_wtime() - before;
    before = omp_get_wtime();
}

    // Count elements per thread
#pragma omp for
    for (int i = 0; i < n; i++) {
        unsigned long long digit = (arr[i] >> exp) & ((1ULL << b) - 1);
        count[thread_id][digit]++;
    }

#pragma omp single
{
    count_time += omp_get_wtime() - before;
    before = omp_get_wtime();
    
    // Create a global count array
    vector<int> global_count(radix, 0);
    vector<vector<int>> thread_offsets(num_threads, vector<int>(radix, 0));
    
    // Sum counts across all threads for each digit
    for (int digit = 0; digit < radix; digit++) {
        int digit_total = 0;
        for (int t = 0; t < num_threads; t++) {
            thread_offsets[t][digit] = digit_total;
            digit_total += count[t][digit];
            global_count[digit] = digit_total;
        }
    }
    
    // Calculate global prefix sum
    int total_pos = 0;
    for (int digit = 0; digit < radix; digit++) {
        int digit_count = global_count[digit];
        global_count[digit] = total_pos;
        total_pos += digit_count;
    }
    
    // Calculate final prefix position for each thread and digit
    for (int t = 0; t < num_threads; t++) {
        for (int digit = 0; digit < radix; digit++) {
            prefix_arr[t][digit] = global_count[digit] + thread_offsets[t][digit];
        }
    }
    
    prefix_time += omp_get_wtime() - before;
    before = omp_get_wtime();
}

    // We need local copies of count to avoid race conditions
    vector<int> local_count(radix);
    for (int digit = 0; digit < radix; digit++) {
        local_count[digit] = count[thread_id][digit];
    }

    // Position elements
#pragma omp for
    for (int i = 0; i < n; i++) {
        unsigned long long digit = (arr[i] >> exp) & ((1ULL << b) - 1);
        if (local_count[digit] > 0) {
            int position = prefix_arr[thread_id][digit];
            output[position] = arr[i];
            prefix_arr[thread_id][digit]++;
            local_count[digit]--;
        }
    }

#pragma omp single
{
    output_time += omp_get_wtime() - before;
    before = omp_get_wtime();
    
    // Copy back to original array
    for (int i = 0; i < n; i++) {
        arr[i] = output[i];
    }
    copyback_time += omp_get_wtime() - before;
}
}
}