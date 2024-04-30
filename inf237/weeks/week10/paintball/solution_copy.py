
def find_path(graph, current, visited):
    visited[current] = True
    if current == 't':
        return True
    for neighbor in graph[current]:
        if not visited[neighbor]:
            if find_path(graph, neighbor, visited):
                return True
    return False

def assign_targets(n, m, edges):
    graph = {'s': [], 't': []}
    for v in range(1, n + 1):
        graph['s'].append(v)
        graph[v] = []
        graph[str(v)] = ['t']
    for a, b in edges:
        graph[a].append(str(b))
        graph[b].append(str(a))
    visited = {node: False for node in graph}
    if find_path(graph, 's', visited):
        targets = [int(node) for node in visited if visited[node] and type(node) == int]
        if len(targets) == n:
            return "\n".join(map(str, targets))
    return "Impossible"

# Sample Input
n, m = map(int, input().split())
edges = [tuple(map(int, input().split())) for _ in range(m)]

# Output
print(assign_targets(n, m, edges))
