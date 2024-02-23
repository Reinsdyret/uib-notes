import math

def bellman_ford(vertices, src, g):
    # Inspired by geeksforgeeks implementation of bellman ford here:
    # https://www.geeksforgeeks.org/bellman-ford-algorithm-dp-23/#algorithm-to-find-negative-cycle-in-a-directed-weighted-graph-using-bellmanford
    dist = {i:float("inf") for i in vertices}
    dist[src] = 0

    for _ in range(len(vertices) - 1):
        for (u, v, w) in g:
            if dist[u] != float("inf") and dist[u] + w < dist[v]:
                dist[v] = dist[u] + w 
    
    # Check for negative cycles
    for (u, v, w) in g:
        if dist[u] != float("inf") and dist[u] + w < dist[v]:
            return True

    return False



inp = input()

while inp.strip() != '0':
    graph = []
    n = int(inp)
    vertices = input().strip().split()

    cases = int(input())
    for case in range(cases):
        u, v, weight = input().strip().split()
        u_weight, v_weight = map(int, weight.split(':'))
        if v_weight == u_weight:
            w = 0
        else:
            w = -1 * math.log(v_weight / u_weight)

        graph.append((u,v, w))
    
    inp = input()

    for (v,_,_) in graph: # Might not need this :) Just need root
        if bellman_ford(vertices, v, graph):
            print("Arbitrage")
            break
    else: # Sorry but not sorry im using else after for
        print("Ok")
    

