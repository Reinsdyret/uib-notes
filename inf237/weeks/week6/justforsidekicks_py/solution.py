left = lambda x: 2 * x
right = lambda x: 2 * x + 1
parent = lambda x: x // 2
index = lambda T, i: len(T) // 2 + i

def fill(tree, op):
    internal = range(1, len(tree) // 2)
    for idx in reversed(internal):
        tree[idx] = op((tree[left(idx)], tree[right(idx)]))

def query_(t, l, r):
    yield t[l]
    if l != r:
        yield t[r]
    while True:
        pl = parent(l)
        pr = parent(r)
        if pl == pr:
            return
        if l % 2 == 0:
            yield t[right(pl)]
        if r % 2 == 1:
            yield t[left(pr)]
        l = pl
        r = pr

def query(t, l, r, op=sum):
    return op(query_(t, l, r))

def update(tree, idx, value, op=sum):
    tree[idx] = value
    idx = parent(idx)
    while idx > 0:
        tree[idx] = op((tree[left(idx)], tree[right(idx)]))
        idx = parent(idx)

def create_tree(data, op=sum):
    t = [0] * len(data) + data
    fill(t, op)
    return t

def q(tree, l, r, op=sum):
    li = index(tree, l)
    ri = index(tree, r)
    return query(tree, li, ri, op)

def s(tree, idx, value, op=sum):
    i = index(tree, idx)
    update(tree, i, value, op)


    
n, q1 = map(int, input().strip().split())

values = list(map(int, input().strip().split()))
gems1 = list(map(int, input()))

ttt = {}
for i, v in enumerate(values):
    ttt[i] = v

gems = []
for g in gems1:
    gems.append(g - 1)

trees = []
for i in range(6):
    tree_arr = [1 if gems[j] == i else 0 for j in range(len(gems))]

    trees.append(create_tree(tree_arr))


for _ in range(q1):
    op, v1, v2 = map(int, input().strip().split())

    if op == 1:
        v1 = v1 - 1
        v2 = v2 - 1
        for i, tree in enumerate(trees):
            if i == v2:
                s(tree, v1, 1)
            else:
                s(tree, v1, 0)
    elif op == 2:
        ttt[v1 - 1] = v2
    else:
        v1 -= 1
        v2 -= 1
        num = 0
        for i, tree in enumerate(trees):
            num += q(tree, v1, v2) * ttt[i]
        print(num)