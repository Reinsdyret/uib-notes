import functools
from collections import defaultdict

def remove_k_mins(data, k):
    minimum_numbers = []
    for [v1, v2] in data:
        #print(x,y)
        minimum_v = min(v1,v2)
        if len(minimum_numbers) == 0 or minimum_v < max(x for (x,_) in minimum_numbers):
            minimum_numbers.append((minimum_v, minimum_v == v1))

        if len(minimum_numbers) > k:
            max_val = 0
            for (x, t) in minimum_numbers:
                if x > max_val:
                    max_val = (x, t)
            minimum_numbers.remove(max_val)

    for (x,t) in minimum_numbers:
        [v1, v2] = data[x][y]
        if t:
            v1 = 0
        else: v2 = 0

    return data



opt = defaultdict(int)
data = []

n, k = map(int,input().split(' '))
inp = "3"
while inp != "":
    for _ in range(n):
        data.append(list(map(int, input().split(' '))))
    inp = input().strip()
    inp = input().strip()

print(data)
print(data[1:-1])
print([data[0]] + remove_k_mins(data[1:-1],k) + [data[-1]])
