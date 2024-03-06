# Intuitive (aka idiotic) solution with some dictionaries

n, q = map(int, input().strip().split())

values = list(map(int, input().strip().split()))
gems1 = list(map(int, input()))

# Create type to value dict
type_to_value = {}
for i, v in enumerate(values):
    type_to_value[i] = v

# Make all gems to 0-indexed
gems = []
for g in gems1:
    gems.append(g - 1)


for _ in range(q):
    op, v1, v2 = map(int, input().strip().split())

    if op == 1:
        gems[v1-1] = v2 - 1
    elif op == 2:
        type_to_value[v1 - 1] = v2
    else:
        v1 = v1 - 1
        num = 0
        for g in range(v1, v2):
            num += type_to_value[gems[g]]
        print(num)
