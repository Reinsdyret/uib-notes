from collections import namedtuple as T

def tarjans(graph):
    sccs = []
    ids  = [None] * len(graph.V)
    lows = [None] * len(graph.V)
    onstack = [False] * len(graph.V)
    index = 0
    stack = []
    for node in graph.V:
        if(ids[node] == None):
            dfs(graph, node, ids, lows, index, stack, onstack, sccs)

    return sccs


def dfs(graph, node, ids, lows, index, stack, onstack, sccs):
    ids[node] = index
    lows[node] = index
    index += 1

    stack.append(node)
    onstack[node] = True

    for neighbour in graph.N[node]:
        if ids[neighbour] == None:
            dfs(graph, neighbour, ids, lows, index, stack, onstack, sccs)
            lows[node] = min(lows[node], lows[neighbour])
        elif onstack[neighbour]:
            lows[node] = min(lows[node], lows[neighbour])

    if ids[node] == lows[node]:
        stackElement = None
        currentScc = []

        while node != stackElement:
            stackElement = stack.pop()
            onstack[stackElement] = False
            currentScc.append(stackElement)

        sccs.append(currentScc)

Graph = T("Graph", "V N")
Interval = T("Interval", "s f")

G = Graph(V=[0, 1, 2, 3,4,5,6,7], N=[
    [1, 3], 
    [2, 4, 6], 
    [0, 6], 
    [2],
    [6,5],
    [6,7],
    [4],
    []])

print(tarjans(G))
