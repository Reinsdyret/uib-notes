def is_valid(num):
    if '0' in str(num):
        return False
    digit_set = set()
    for i in str(num):
        if num % int(i) != 0:
            return False
        digit_set.add(i)
    
    return len(digit_set) == len(str(num))

a, b = map(int, input().strip().split())

valids = 0
for num in range(a, b):
    if is_valid(num):
        valids += 1

print(valids)