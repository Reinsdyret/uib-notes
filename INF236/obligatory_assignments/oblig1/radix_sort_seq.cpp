#include "mt19937-64.c"
#include <iostream>
#include <vector>
#include <cmath>
#include <omp.h>
using namespace std;

void counting_sort(vector<unsigned long long> arr, int n, int exp);


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

  return 0;
}


void counting_sort(vector<unsigned long long> arr, int n, int exp, int b) {

}
