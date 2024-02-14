n = int(input().strip())

numbers = list(map(int, input().strip().split()))

maxHeight = max(numbers)
nestMaxHeight = None
p = 0
m = 0
for num in numbers:
    if num == maxHeight:
        p += 1
    else:
        if nestMaxHeight == None or num > nestMaxHeight:
            nestMaxHeight = num
        m += 1

if nestMaxHeight == None:
    print(maxHeight)
    exit()
if nestMaxHeight < (maxHeight - p) or maxHeight < n:
    if maxHeight < n:
        print(maxHeight)
    else:
        print(nestMaxHeight + p)
else:
    print(min(nestMaxHeight + p, n))