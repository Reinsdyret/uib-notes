from collections import namedtuple as T


class TarjansGraphUtil:
    """
    A class to hold information regarding the tarjans algorithm.

    ...

    Attributes
    ----------
    graph: namedtuple
        the graph to execute tarjans algorithm on 
    index : int 
        an index used to number each vertex
    ids : dictionary
        A dictionary of the ids for each vertex.
        This id is related to the index attribute. values are ints.
    lows : dictionary 
        A dictionary, symbolizing the lowest connected vertex for each vertex. Values are ints.
    onstack : dictionary 
        A dictionary keeping track for each vertex if it is on the stack. Values are bool.
        This is used to check if a vertex is on the stack in constant time.
    stack : list
        A list that is used as a stack.
    sccs : list 
        A list of the strongly connected components in the graph, after the algorithm has run.

    The attributes ids, lows and onstack are all pretty similar with the key being the index of the vertex in the list in the graph. 
    These are dictionaries because lookup and update are amortized O(1). Since we are only using bool values and ints we can ignore the worst case of O(n) because this only occurs when we either use multiple values per key and/or have a bad hashing algorithm.
    """
    def __init__(self, graph):
        """
        Constructs all necessary attributes to later compute tarjans algorithm.

        Parameters
        ----------
        graph : namedtuple
            A namedtuple on the form
            graph.V is an array of vertices
            graph.N is an array of the neighbours for each vertex.
            
            e.g. the graph = graph(V=[0, 1], N=[1])
            Here vertex 0 has an edge to 1, but 1 has no edges going out.

        Assumptions
        -----------
        The graph represents a directed graph.
        """
        self.graph = graph
        self.index = 0
        self.ids = dict()
        self.lows = dict()
        self.onstack = dict()
        for v in range(len(graph.V)):
            self.ids[v] = None
            self.lows[v] = None
            self.onstack[v] = False
        self.stack = []
        self.sccs = []


def find_scc_tarjans(G):
    """
    Returns a list of the strongly connected components as lists, uting tarjans algorithm.
        
        Parameters:
            G (TarjansGraphUtil): 
                An instance of the TarjansGraphUtil. 
                This class holds all necessary information for the tarjans algorithm.
        Returns:
            sccs (list): A list of all the strongly connected components as lists.
    """
    for i in range(len(G.graph.V)):
        if G.ids[i] is None:
            dfs(G, i)

    return G.sccs

def dfs(G, current):
    """
    Runs dfs on the graph in G and updates the sccs list when it finds an scc.

        Parameters:
            G (TarjansGraphUtil): 
                An instance of the TarjansGraphUtil. 
                This class holds all necessary information for the tarjans algorithm.
            current (int):
                the index of the vertex currently visiting.
        Does not return anything but after find_scc_tarjans has ran and used this. The G.sccs list should contain all strongly connected components in the graph.

    Runs in `O(V + E)` where V = total number of vertices and E = total number of edges
    """
    G.ids[current] = G.index
    G.lows[current] = G.index
    G.index += 1

    G.stack.append(current)
    G.onstack[current] = True

    for neighbour in G.graph.N[current]:
        if G.ids[neighbour] is None:
            dfs(G, neighbour)
            G.lows[current] = min(G.lows[current], G.lows[neighbour])
        elif G.onstack[neighbour]:
            G.lows[current] = min(G.lows[current], G.ids[neighbour])

    if G.ids[current] == G.lows[current]:
        scc = []
        while True:
            w = G.stack.pop()
            G.onstack[w] = False
            scc.append(w)
            if w == current:
                break
        G.sccs.append(scc)



