
def dfs(v, adj_dict, visited):
    visited.add(v)

    for n in adj_dict[v]:
        if n not in visited:
            dfs(n, adj_dict, visited)

    return visited


n_boxes = int(input())
boxes = map(int, input().split())

box_graph = {i: [] for i in range(n_boxes + 1)}

for i, box in enumerate(boxes):
    if box == 0:
        continue

    box_graph[box].append(i + 1)

q = int(input())

for _ in range(q):
    queries = map(int, input().split())
    visited = set()
    first = True
    for query in queries:
        if first:
            first = False
            continue

        visited = dfs(query, box_graph, visited)

    print(len(visited))
    #print(visited) 
    #print(box_graph[2])

