import collections
# This is just union find to find connected components (white components)



class UnionFind:
    def __init__(self, size):
        self.parent = list(range(size))
        self.rank = [0] * size

    def find(self, x):
        if self.parent[x] != x:
            self.parent[x] = self.find(self.parent[x])
        return self.parent[x]

    def union(self, x, y):
        rootX = self.find(x)
        rootY = self.find(y)
        if rootX != rootY:
            if self.rank[rootX] > self.rank[rootY]:
                self.parent[rootY] = rootX
            elif self.rank[rootX] < self.rank[rootY]:
                self.parent[rootX] = rootY
            else:
                self.parent[rootY] = rootX
                self.rank[rootX] += 1

# Graph class
class UndirectedGraph:
    def __init__(self, vertices, n, m):
        self.vertices = vertices
        self.n = n
        self.m = m
        self.uf = UnionFind(n * m)
        self.blacked = set()

    def index(self, x, y):
        """Converts 2D coordinates to a single index."""
        return x * self.m + y

    def remove_edges_to(self, v):
        """Marks cell as black but does not union it with anything."""
        self.blacked.add(v)
        self.update_components_after_blackening(v[0], v[1])
        # No need to directly manipulate the union-find structure for black cells

    def update_components_after_blackening(self, x, y):
        """Updates connected components considering (x, y) turning black."""
        possible_neighbours = [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
        for i, (nx, ny) in enumerate(possible_neighbours):
            if 0 <= nx < self.n and 0 <= ny < self.m and (nx, ny) not in self.blacked:
                for j in range(i + 1, len(possible_neighbours)):
                    mx, my = possible_neighbours[j]
                    if 0 <= mx < self.n and 0 <= my < self.m and (mx, my) not in self.blacked:
                        root1 = self.uf.find(self.index(nx, ny))
                        root2 = self.uf.find(self.index(mx, my))
                        if root1 != root2:
                            self.uf.union(root1, root2)

    def connected_components(self):
        """Counts the number of white connected components."""
        component_roots = set()
        for x in range(self.n):
            for y in range(self.m):
                if (x, y) not in self.blacked:
                    root = self.uf.find(self.index(x, y))
                    component_roots.add(root)
        return len(component_roots)

        

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
