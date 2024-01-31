# Tests the possible solution of:
# The largest weight is erither:
#   A weight where its mate is on another row
#   A weight where surrounding weights are larger than it

# PLAN:
# Make a function finding the largest weight on different rows.
# Make a function finding the largest weight surrounded by larger weights (Recursive?)
#   Goes through list and pops element if it is surrounded by larger weights

def largest_diff_row(row1, row2):
    largest = 0
    for weight in row1:
        if weight > largest and weight in row2:
            largest = weight
    
    return largest

def largest_surrounded(row):
    largest = 0
    for i in range(1,len(row)-1):
        if row[i] > largest and row[i] < row[i-1] <= row[i+1]:
            largest = row[i]
    
    return largest

n = input()

row1 = [int(x) for x in input().split(' ')]
row2 = [int(x) for x in input().split(' ')]

print(max(largest_diff_row(row1, row2), max(largest_surrounded(row1), largest_surrounded(row2))))
