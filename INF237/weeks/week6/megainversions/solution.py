def merge_sort_counting_inversions(arr):
    if len(arr) <= 2:
        return arr, 0
    
    len_divide_3 = len(arr)//3
    left  = arr[:len_divide_3]
    middle = arr[len_divide_3+1:len_divide_3*2]
    right = arr[len_divide_3*2+1:]

    left, lefti = merge_sort_counting_inversions(left)
    middle, middlei = merge_sort_counting_inversions(middle)
    right, righti = merge_sort_counting_inversions(right)
    temp = []

    i = j = k = 0
    count = lefti + righti + middlei

    while i < len(left) and j < len(right) and k < len(middle):
        if left[i] <= middle[k] <= right[j]:
            temp.append(left[i])
            i += 1
        else:
            temp.append(right[j])
            j += 1
            count += len(left) - i 

    temp += left[i:]
    temp += middle[k:]
    temp += right[j:]

    return temp, count

n = int(input())

arr = list(map(int, input().split()))

print(merge_sort_counting_inversions(arr)[1])