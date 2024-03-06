# Segment tree code, written og Pål Grønås's slides from inf237
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

n, k = map(int, input().strip().split())
tree = create_tree([0] * n)
#print(tree)

for _ in range(k):
    stuff = input().strip().split()
    if stuff[0] == 'F':
        idx = int(stuff[1]) - 1
        value = tree[index(tree, idx)]
        if value >= 1:
            s(tree, idx, 0)
        else:
            s(tree, idx, 1)
    else:
        l, r = (stuff[1], stuff[2])
        print(q(tree, int(l) - 1, int(r) - 1))

n, q = map(int, input().split())
values = list(map(int, input().split()))
