import functools

@functools.cache
def find_coin_room(v):
    global vertices
    global visited
    steps = 0
    while vertices[v] != -1:
        v = vertices[v]
        if v in visited:
            return 0, -1
        visited.add(v)
        steps += 1
    return v, steps

n = int(input())
vertices = list(map(int, input().split(' ')))

luigi_n = int(input())
luigi_coins = list(map(int, input().split(' ')))

not_pipes = set()
# Fill non pipe set
for i in range(len(vertices)):
    not_pipes.add(vertices[i])

pipes = []
for i in range(len(vertices)):
    if i not in not_pipes and i not in luigi_coins:
        pipes.append(i)

best_distance_coins = {}
luigi_room_order = []
visited = set()
for x in range(len(luigi_coins)):
    room, steps = find_coin_room(luigi_coins[x])
    best_distance_coins[room] = (luigi_coins[x], steps)
    luigi_room_order.append(room)

# Explore remaizning pipes:
for pipe in pipes:
    visited = set()
    room, steps = find_coin_room(pipe)
    if steps == -1:
        continue
    if room not in best_distance_coins:
        continue
    saved_pipe, saved_steps = best_distance_coins[room]
    if steps < saved_steps or (steps == saved_steps and pipe < saved_pipe):
        best_distance_coins[room] = (pipe, steps)

# Output
for room in luigi_room_order:
    p, s = best_distance_coins[room]
    print(p)

