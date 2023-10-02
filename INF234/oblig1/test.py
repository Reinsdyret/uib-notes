from collections import namedtuple as T
#from TarjanSccSolverAdjacencyList import Tarjans
#from gptSvar import TarjanSccSolverAdjacencyList
from tarjans import *
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


t1 = TarjansGraphUtil(G)
sccs = find_scc_tarjans(t1)
print(sccs)

