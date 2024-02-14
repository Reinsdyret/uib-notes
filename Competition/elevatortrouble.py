
f, s, g, u , d = map(int, input().strip().split())

if (u == 0 or d == 0) and g != s:
    print("take the stairs")
    exit()

if g < s:
    res = (s - g) % d
    if res == 0:
        print((s - g) // d)
        exit()

    if d == 0:
        print("take the stairs")
        exit()
    print(res // u)

if g > s:
    res = (g - s) % u 
    if res == 0:
        print((g - s) // u)
        exit()

    if d == 0:
        print("take the stairs")
        exit()
    print("cam here")
    print(res // d)

if g == s:
    print("0")
    exit()

