#include <iostream>
#include <vector>
#include <ctime>
using namespace std;

struct date {
  int day;
  int month;
  int year;
};

void sort (vector<int> &nums);
void sort (vector<date> &nums);

int main (int argc, char *argv[]) {
  srand ( time(NULL) );

  vector<int> nums = { 50, 64, 24, 16, 80, 30, 2 };
  sort(nums);
  cout << "Nums sorted \n";

  for (int num: nums)
    cout << " " << num;

  vector<date> dates = {};

  for (size_t i = 0; i < 5; i++) {
    struct date day;
    day.day = rand() % 30;
    day.month = rand() % 12;
    day.year = rand() % 1;
    dates.push_back(day);
  }

  sort(dates);

  cout << "Dates sorted \n";
  for (struct date date : dates) {
    cout << date.day << "." << date.month << "." << date.year << "; ";
  }
  cout << "\n";

  return 0;
}


void sort (vector<int> &nums) {
  int n = nums.size();

  for (size_t i = 0; i < n; i++) {
    bool swapped = false;

    for (size_t j = 0; j < n-i-1; j++) {
      if (nums[j] > nums[j+1]){
        swap(nums[j], nums[j+1]);
        swapped = true; 
      }
    }

    if (!swapped)
      break;
  }
}

void sort (vector<date> &nums) {
  int n = nums.size();

  for (size_t i = 0; i < n; i++) {
    bool swapped = false;

    for (size_t j = 0; j < n-i-1; j++) {
      int day1 = nums[j].day;
      int day2 = nums[j+1].day;
      int month1 = nums[j].month;
      int month2 = nums[j+1].month;
      int year1 = nums[j].year;
      int year2 = nums[j+1].year;

      if (year1 > year2) {
        cout << "\n swapping " << year1 << " and " << year2;
        swap(nums[j], nums[j+1]);
        swapped = true;
      } else if (year1 == year2 && (month1 > month2 || (month1 == month2 && day1 > day2))) {
        swap(nums[j], nums[j+1]);
        swapped = true;
      } else {
        date date1 = nums[j];
        date date2 = nums[j+1];
        cout << "\n" << date1.day << "." << date1.month << "." << date1.year << " is smaller than " << date2.day << "." << date2.month << "." << date2.year << ";";
      }
    }

    if (!swapped)
      break;
  }
}
