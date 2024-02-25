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
        self.blacked = set()
        
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
        for u in self.vertices:
            self.graph[u][v] = False
            self.graph[v][u] = False
        self.blacked.add(v)

    def neighbours(self,u):
        n = []
        for (v, is_edge) in self.graph[u].items():
            if is_edge:
                n.append(v)

        return n

    def connected_components(self):
        visited = {}
        
        for v in vertices:
            visited[v] = False

        number_of_connected = 0
        
        for v in vertices:
            if not visited[v] and v not in self.blacked:
                self.dfs(visited, v)
                number_of_connected += 1
        
        return number_of_connected

        
    def dfs(self, visited, src):
        visited[src] = True

        for n in self.neighbours(src):
            if not visited[n] and n not in self.blacked:
                self.dfs(visited, n)

        

# Take in input and create graph
n, m, q = map(int, input().strip().split())

vertices = [(x, y) for x in range(n) for y in range(m)]
G = UndirectedGraph(vertices, n, m)

# Take in the deletion of edges
for _ in range(q):
    x1, y1, x2, y2 = map(int, input().strip().split())
    
    for x in range(x1, x2 + 1):
        for y in range(y1, y2 + 1):
            G.remove_edges_to((x - 1, y - 1))
    
    print(G.connected_components())

#for v in G.neighbours((1,1)):
#    print(f"(1,1) to {v} = {G.graph[(1,1)][v]}")

#G.remove_edges_to((1,1))
#print("Removed edges")

#for v in G.neighbours((1,1)):
#    print(f"(1,1) to {v} = {G.graph[(1,1)][v]}")
