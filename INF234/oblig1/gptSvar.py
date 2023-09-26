from collections import deque, namedtuple

Graph = namedtuple("Graph", "V N")

class TarjanSccSolverAdjacencyList:

    def __init__(self, graph_obj):
        if graph_obj is None:
            raise ValueError("Graph cannot be None.")
        
        self.graph = graph_obj
        self.n = len(graph_obj.V)
        self.solved = False
        self.sccCount = 0
        self.id = 0
        self.visited = None
        self.ids = None
        self.low = None
        self.sccs = None
        self.stack = None
        self.UNVISITED = -1

    def sccCount(self):
        if not self.solved:
            self.solve()
        return self.sccCount

    def getSccs(self):
        if not self.solved:
            self.solve()
        return self.sccs

    def solve(self):
        if self.solved:
            return

        self.ids = [self.UNVISITED] * self.n
        self.low = [0] * self.n
        self.sccs = [0] * self.n
        self.visited = [False] * self.n
        self.stack = deque()

        for i in range(self.n):
            if self.ids[i] == self.UNVISITED:
                self.dfs(i)

        self.solved = True

    def dfs(self, at):
        self.ids[at] = self.low[at] = self.id
        self.id += 1
        self.stack.append(at)
        self.visited[at] = True

        for to in self.graph.N[at]:
            if self.ids[to] == self.UNVISITED:
                self.dfs(to)
            if self.visited[to]:
                self.low[at] = min(self.low[at], self.low[to])

        if self.ids[at] == self.low[at]:
            while True:
                node = self.stack.pop()
                self.visited[node] = False
                self.sccs[node] = self.sccCount
                if node == at:
                    break
            self.sccCount += 1


G = Graph(V=[0, 1, 2, 3], N=[[0], [0, 2, 3], [0, 1], [1]])
solver = TarjanSccSolverAdjacencyList(G)
print("Number of SCCs:", solver.sccCount)
print("SCCs:", solver.getSccs())
