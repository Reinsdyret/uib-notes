import collections
# This is just union find to find connected components (white components)


# Graph class
class UndirectedGraph:
    graph = {}

    def __init__(self, vertices, n, m):
        """ Runtime O(v) """
        self.vertices = vertices
        #self.edges = edges
        self.edges = []
        
        for v in vertices:
            # Adding all edges to all nodes
            self.graph[v] = {}
            x, y = v
            possible_neighbours = (
                (x + 1, y),
                (x, y + 1),
                (x - 1, y),
                (x, y - 1)
            )

            for (nx, ny) in possible_neighbours:
                if 0 <= nx < n and 0 <= ny < m:
                    self.graph[v][(nx, ny)] = True
                    self.edges.append((v, (nx, ny)))
        



    def remove_edges_to(self, v):
        """ Removes edge to v """
        for x in self.vertices:
            if (x,v) in self.edges:
                self.edges.remove((x,v))
            if (v,x) in self.edges:
                self.edges.remove((v,x))

    def neighbours(self,u):
        n = []
        for (v, is_edge) in self.graph[u].items():
            if is_edge:
                n.append(v)

        return n

    def connected_components(self):
        # Inspired and taken from https://www.geeksforgeeks.org/connected-components-in-an-undirected-graph/
        parent = {x: x for x in vertices}
        
        for (u, v) in self.edges:
            parent[self.merge(parent, u)] = self.merge(parent, v)
            
        number_of_connected = 0
        
        for v in vertices:
            if parent[v] == v:
                number_of_connected += 1
        
        
        return number_of_connected

        
    def merge(self, parent, x):
        # Inspired and taken from https://www.geeksforgeeks.org/connected-components-in-an-undirected-graph/
        if parent[x] == x:
            return x
        else:
            # Path compression
            return self.merge(parent, parent[x])

        

# Take in input and create graph
n, m, q = map(int, input().strip().split())

vertices = [(x, y) for x in range(n) for y in range(m)]
G = UndirectedGraph(vertices, n, m)

# Take in the deletion of edges
for _ in range(q):
    x1, y1, x2, y2 = map(int, input().strip().split())
    
    for x in range(x1, x2 + 1):
        for y in range(y1, y2 + 1):
            G.remove_edges_to((x, y))
    
    print(G.connected_components())

