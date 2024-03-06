def merge_sort_counting_inversions(arr):
    if len(arr) == 1:
        return arr, 0
    
    left  = arr[:len(arr)//2]
    right = arr[len(arr)//2:]

    left, lefti = merge_sort_counting_inversions(left)
    right, righti = merge_sort_counting_inversions(right)
    temp = []

    i = j = 0
    count = lefti + righti

    while i < len(left) and j < len(right):
        if left[i] <= right[j]:
            temp.append(left[i])
            i += 1
        else:
            temp.append(right[j])
            j += 1
            count += len(left) - i 

    temp += left[i:]
    temp += right[j:]

    return temp, count

n = int(input())

arr = [int(input()) for _ in range(n)]

print(merge_sort_counting_inversions(arr)[1]) 