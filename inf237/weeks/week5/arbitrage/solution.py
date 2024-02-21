
def find_cycle(u, goal, visited, path, paths, g, first_iteration):

    visited.add(u)

    print(f"{u = }, {goal = }, {first_iteration = }")
    if u == goal and not first_iteration:
        paths.append(path)
        print(path)
        return paths
    else:
        for (v, (u_weight, v_weight)) in g[u]:
            #print(v)
            if v not in visited:
                path.append((u_weight, v_weight))
                paths.extend(find_cycle(v, goal, visited, path, paths, g, False))
    
    path.pop()
    visited.remove(u)

    return paths





inp = input()

while inp.strip() != '':
    graph = {}
    n = int(inp)
    vertices = input().strip().split()

    cases = int(input())
    for case in range(cases):
        u, v, weight = input().strip().split()
        u_weight, v_weight = map(int, weight.split(':'))

        if u not in graph:
            graph[u] = [(v, (u_weight,v_weight))]
        else:
            graph[u].append((v, (u_weight, v_weight)))
    
    inp = input()
    print(find_cycle('CZK', 'CZK', set(), [], [], graph, True))

