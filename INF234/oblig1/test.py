from collections import namedtuple as T
#from TarjanSccSolverAdjacencyList import Tarjans
#from gptSvar import TarjanSccSolverAdjacencyList
from tarjans import Tarjans
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

for v in G.V:
    print(v, G.N[v])
intervals = [Interval(1, 3), Interval(2, 4), Interval(1, 4)]
for I in intervals:
    for J in intervals:
        if I.s < J.s < I.f:
            print(f"{I} overlaps {J}")

t1 = Tarjans(G)

print(t1.find_scc_tarjans())

