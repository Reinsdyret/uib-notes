f, s, g, u , d = map(int, input().strip().split())


if g == s:
    print("0")
    exit()

steps = 0
visited_floors = set()
curr_floor = s
while True:
    visited_floors.add(curr_floor)
    if curr_floor > g:
        if d == 0:
            print("use the stairs")
            exit()
        while True:
            curr_floor -= d
            steps += 1
            #print(curr_floor)
            if curr_floor < 1:
                print("use the stairs")
                exit()
            if curr_floor not in visited_floors:
                break
            
    if curr_floor < g:
        if u == 0:
            print("use the stairs")
            exit()
        while True:
            curr_floor += u
            steps += 1
            #print(curr_floor)
            if curr_floor > f+u:
                print("use the stairs")
                exit()
            if curr_floor not in visited_floors:
                break
            

    if curr_floor == g:
        break
print(steps)
            