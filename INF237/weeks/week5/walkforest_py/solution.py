def countPaths(u, goal, visited, count, g):
    visited[u] = True

    if u == goal:
        count += 1
    
    else:
        for (neighbour, w) in g[u]:
            count += countPaths(neighbour, goal, visited, count, g)
    
    visited[u] = False

    return count



inp = input().strip()

while inp != '0':
    n, m = map(int,inp.split())
    graph = {}

    for _ in range(m):
        a, b, w = map(int, input().split())
        if a in graph:
            graph[a].append((b, w))
        else:
            graph[a] = [(b, w)]

    
    count = countPaths(1, 2, [False] * (n + 1), 0, graph)
    print(count)

    inp = input().strip()
    