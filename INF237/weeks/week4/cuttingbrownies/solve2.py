from math import log2

def can_solve(b, d, p):
    lb = int(log2(b))
    ld = int(log2(d))

    if p == "Vicky":
        if lb > ld:
            return "can win"
        else: return "cannot win"
    else:
        if ld > lb:
            return "can win"
        else: return "cannot win"

N = int(input())

for _ in range(N):
    b, d, p = input().strip().split(' ')
    print(f"{p} {can_solve(int(b), int(d), p)}")
