def explorepathts(root, graph, got_shot_by, visited):
    if visited[root]:
        return False

    visited[root] = True

    if got_shot_by[root] != -1 and got_shot_by[root] != root:
        return True # Already shot 

    for neigh in graph[root]:
        if not visited[neigh] and explorepathts(neigh, graph, got_shot_by, visited):
            got_shot_by[root] = neigh
            return True
    return False

n, m = map(int, input().split())

graph = {}
got_shot_by = {}

for v in range(1, n+1):
    graph[v] = []
    got_shot_by[v] = -1


for _ in range(m):
    a, b = map(int, input().split())
    graph[a].append(b)
    graph[b].append(a)


for i in range(1, n + 1):
    visited = [False] * (n + 1)
    if not explorepathts(i, graph, got_shot_by, visited):
        print("Impossible")
        print("HERE")
        exit()

for i in range(1, n+1):
    if got_shot_by[i] == -1:
        print("Impossible")
        exit()

for i in range(1, n+1):
    print(got_shot_by[i])


