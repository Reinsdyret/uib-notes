#include "mt19937-64.c"
#include <iostream>
#include <vector>
#include <cmath>
#include <omp.h>
#include <algorithm>
#include <cstring>
using namespace std;

// Padding prevent false sharing
#define CACHE_LINE 64
#define PADDING (CACHE_LINE / sizeof(int))

// Forward declarations
void radix_sort(vector<unsigned long long>& arr, int n, int b);

int main(int argc, char *argv[]) {
    int n;
    cout << "N: ";
    cin >> n;

    int b;
    cout << "B: ";
    cin >> b;

    if (b <= 0) {
        cout << "Invalid B, please only give B that is larger than 0.\n";
        return 0;
    }

    cout << "Registered N = " << n << " and B = " << b << "\n";

    double before = omp_get_wtime();

    vector<unsigned long long> nums(n);
    
    for (int i = 0; i < n; i++) {
        nums[i] = genrand64_int64();
    }
    cout << "Time spent generating numbers: " << omp_get_wtime() - before << "\n";
    
    radix_sort(nums, n, b);

    // Verify array is sorted
    bool is_sorted = true;
    for (int i = 1; i < n; i++) {
        if (nums[i] < nums[i-1]) {
            is_sorted = false;
            break;
        }
    }
    cout << "Array is " << (is_sorted ? "sorted" : "not sorted") << "\n";
    cout << nums[0] << endl;

    return 0;
}

void radix_sort(vector<unsigned long long>& arr, int n, int b) {
    int num_passes = (64 + b - 1) / b; 
    int radix = 1 << b;
    
    int thread_count;
    #pragma omp parallel
    {
        thread_count = omp_get_num_threads();
    }
    omp_set_num_threads(thread_count);
    
    vector<unsigned long long> output(n);
    
    // counting arrays with padding to avoid false sharing
    vector<vector<int>> count(thread_count, vector<int>(radix + PADDING, 0));
    vector<vector<int>> prefix(thread_count, vector<int>(radix + PADDING, 0));
    
    // Timing variables
    double total_time = 0;
    double count_time = 0, prefix_time = 0, placing_time = 0;
    double before;
    double pass_start;
    
    double algorithm_start = omp_get_wtime();
    
    for (int pass = 0; pass < num_passes; pass++) {
        int exp = pass * b;
        int mask = (1ULL << b) - 1;
        #pragma omp parallel
        {
        int thread_id = omp_get_thread_num();
        #pragma omp single
        {
            pass_start = omp_get_wtime();
            
        }
        
        // Clear counts
        #pragma omp for schedule(static)
        for (int t = 0; t < thread_count; t++) {
            memset(count[t].data(), 0, radix * sizeof(int));
        }
        
        #pragma omp single
        before = omp_get_wtime();
        
        // Count elements
        
        
        #pragma omp for schedule(static)
        for (int i = 0; i < n; i++) {
            unsigned long long digit = (arr[i] >> exp) & mask;
            count[thread_id][digit]++;
        }
        
        #pragma omp single
        {
            count_time += omp_get_wtime() - before;
            before = omp_get_wtime();

            int running_sum = 0;
            // Calculate per-thread prefix
            for (int digit = 0; digit < radix; digit++) {
                for (int t = 0; t < thread_count; t++) {
                    prefix[t][digit] = running_sum;
                    running_sum += count[t][digit];
                }
            }
            
            prefix_time += omp_get_wtime() - before;
            before = omp_get_wtime();
        }
        
        // place elements to their new positions
        #pragma omp for schedule(static)
        for (int i = 0; i < n; i++) {
            unsigned long long val = arr[i];
            unsigned long long digit = (val >> exp) & mask;
            output[prefix[thread_id][digit]++] = val;
        }

        #pragma omp single
        {
            placing_time += omp_get_wtime() - before;
            
            // Swap source and destination
            swap(arr, output);
            
            total_time += omp_get_wtime() - pass_start;
        }
    }
    }
    
    // Stop timing the algorithm
    double algorithm_end = omp_get_wtime();
    
    cout << "Counting time: " << count_time << "s\n"
         << "Prefix time: " << prefix_time << "s\n"
         << "Placing time: " << placing_time << "s\n"
         << "Total sort time: " << (algorithm_end - algorithm_start) << "s\n";
}
