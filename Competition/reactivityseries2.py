n, k = map(int, input().strip().split())

neighbours = {}
reversed_neighbours = {}
roots = []
for _ in range(k):
    a,b = map(int, input().strip().split())
    if a in neighbours:
        neighbours[a].append(b)
    else:
        neighbours[a] = [b]

    if b in reversed_neighbours:
        reversed_neighbours[b].append(a)
    else:
        reversed_neighbours[b] = [a]

    if a not in reversed_neighbours:
        reversed_neighbours[a] = []
    
    if b not in neighbours:
        neighbours[b] = []

for (key, value) in reversed_neighbours.items():
    #print(key, value)
    if len(value) == 0:
        roots.append(key)

if len(roots) > 1:
    print("back to the lab")
    exit(0)

    
visited = set()
path = []

# No roots
if len(roots) == 0:
    print("back to the lab")
    exit(0)
node = roots[0]
#print(neighbours)

# Loop from root
finished = False
while len(path) < n and not finished:
    #print(node)
    if len(neighbours[node]) > 1:
        print("back to the lab")
        exit(0)
    if len(neighbours[node]) == 0:
        path.append(node)
        finished = True
        break

    path.append(node)
    node = neighbours[node][0] 

if finished:
    if len(path) != n:
        print("back to the lab")
    else:
        print(" ".join(map(str, path)))
else:
    print("back to the lab")
