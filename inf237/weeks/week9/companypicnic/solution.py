import networkx as nx
G = nx.Graph()

n = int(input())
edges = []
nodes = {}
for _ in range(n):
    a, w, b = input().split()
    w = float(w)
    nodes[a] = w
    G.add_node(a)
    if b != "CEO":
        edges.append((a,b))

for (a,b) in edges:
    weight = min(nodes[a], nodes[b])
    G.add_edge(a,b,weight=weight)
    

#G.add_weighted_edges_from(edges)
max_match = nx.max_weight_matching(G)

avg_match = 0
for (a,b) in max_match:
    weight = min(nodes[a], nodes[b])
    avg_match += weight

avg_match /= len(max_match)

print(f"{len(max_match)} {avg_match}")