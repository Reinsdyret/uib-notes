from decimal import *
getcontext().prec = 10

def solve(v, skip, graph, memo, speed):
    if len(graph[v]) == 0: return (0, 0) # If bottom of the tree, return 0 teams and 0 speed under
    if (v, skip) in memo: return memo[(v, skip)]

    best_teams = 0
    best_speed = 0 
    for neighbour in graph[v]:
        skip_teams, skip_speed = solve(neighbour, True, graph, memo, speed)
        best_teams += skip_teams
        best_speed += skip_speed

    best = (best_teams, best_speed)
    
    if skip:
        for neighbour in graph[v]:
            skip_teams, skip_speed = solve(neighbour, True, graph, memo, speed)
            taken_teams, taken_speed = solve(neighbour, False, graph, memo, speed)

            teams = best_teams - skip_teams + taken_teams + 1
            speeds = best_speed - skip_speed + min(speed[v], speed[neighbour]) + taken_speed

            best = max(best, (teams, speeds))

    memo[(v, skip)] = best 
    return best


n = int(input())
adjlist = {}
memory = {}
speed = {}

for _ in range(n):
    a, w, b = input().split()
    w = float(w)
    if b not in adjlist:
        adjlist[b] = [a]
    else:
        adjlist[b].append(a)

    if a not in adjlist:
        adjlist[a] = []
    speed[a] = w

teams, sums = solve('CEO', False, adjlist, memory, speed)
print(f"{teams} {sums / teams}")
