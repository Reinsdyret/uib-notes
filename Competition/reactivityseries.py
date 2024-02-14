from collections import defaultdict


class DAG:
    def init(self):
        self.vertices = set()
        self.active = 0
        self.ins = defaultdict(list)
        self.outs = defaultdict(list)
        self.activens = defaultdict(int)
        self.source = []

    def contains(self, v):
        return v in self.vertices

    def add_vertex(self, v):
        if not self.contains(v):
            self.vertices.add(v)
            self.active += 1

    def add_edge(self, v, u):
        self.ins[u].append(v)
        self.outs[v].append(u)
        self.active_ns[u] += 1

    def disable(self, v):
        self.active -= 1
        for u in self.outs[v]:
            self.active_ns[u] -= 1
            if self.active_ns[u] == 0:
                self.source.append(u)

    def get_source(self):
        if len(self.source) > 0:
            s = self.source[0]
            self.source.pop(0)
            return s

    def init_source(self):
        for n in self.vertices:
            if self.active_ns[n] == 0:
                self.source.append(n)


def topsort(dag, ordering):
    if dag.active <= 0:
        return ordering

    if len(dag.source) > 1:
        return ordering

    source = dag.get_source()
    ordering.append(source)
    dag.disable(source)
    return topsort(dag, ordering)


def main():
    n, k = map(int, input().split())
    dag = DAG()

    for _ in range(k):
        v, u = map(int, input().split())
        dag.add_vertex(v)
        dag.add_vertex(u)
        dag.add_edge(v, u)

    ordering = []
    dag.initsource()
    topsort(dag, ordering)

    if len(ordering) < n:
        print("back to the lab")
    else:
        print(" ".join(map(str, ordering)))



main()