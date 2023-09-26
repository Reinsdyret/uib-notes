from collections import namedtuple as T


class Tarjans:
    def __init__(self, graph):
        self.graph = graph
        self.index = 0
        self.sccCount = 0
        self.ids  = [None] * len(graph.V)
        self.lows = [None] * len(graph.V)
        self.onstack = [False] * len(graph.V)
        self.stack = []
        self.sccs = []


    def find_scc_tarjans(self):
        for i in self.graph.V:
            if (self.ids[i] == None):
                self.dfs(i)

        return self.lows, self.ids, self.sccs

    def dfs(self, current):
        self.ids[current] = self.index
        self.lows[current] = self.index
        self.index += 1

        self.stack.append(current)
        self.onstack[current] = True

        for neighbour in self.graph.N[current]:
            print(f"Current: {current}, neighbour: {neighbour}")
            if self.ids[neighbour] == None:
                self.dfs(neighbour)

                print(f"Current lows is: {self.lows[current]}, neighbour lows is: {self.lows[neighbour]}, because neighbour is {neighbour}")
                self.lows[current] = min(self.lows[current], self.lows[neighbour])
                print(f"New lows is: {self.lows[current] = }")
                print(f"Because of: {min(self.lows[current], self.lows[neighbour]) = }")
            elif self.onstack[neighbour]:
                print(f"Current lows is: {self.lows[current]}, neighbour lows is: {self.lows[neighbour]}, because neighbour is {neighbour}")
                self.lows[current] = min(self.lows[current], self.lows[neighbour])
                print(f"New lows is: {self.lows[current]}")
                print(f"Because of: {min(self.lows[current], self.lows[neighbour]) = }")

        if self.ids[current] == self.lows[current]:
            w = None
            temp = []
            while current != w:
                w = self.stack.pop()
                print("New scc: ")
                self.onstack[w] = False
                print(f"{w} on scc")
                temp.append(w)
            self.sccs.append(temp)



