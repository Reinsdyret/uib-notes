def rotate_at(word, index):
    rotater = {
            'A': 'B',
            'B': 'C',
            'C': 'D',
            'D': 'E',
            'E': 'F',
            'F': 'A'
            }

    return word[:index] + rotater[word[index]] + (word[index+1:] if index != 7 else "")

def a(word, index):
    if index == 0:
        return rotate_at(word, 1)
    if index == 7:
        return rotate_at(word, 6)

    return rotate_at(rotate_at(word, index + 1), index - 1)

def b(word, index):
    if index == 0 or index == 7:
        return word

    return word[:index + 1] + word[index-1] + word[index+2:]

def c(word, index):
    return rotate_at(word, 7 - index)

def d(word, index):
    if index < 4:
        for i in range(0,index):
            word = rotate_at(word, i)

    if index >= 4:
        for i in range(index + 1, len(word)):
            word = rotate_at(word, i)

    return word

def e(word, index):
    if index == 0 or index == 7:
        return word

    diff = index if index < 4 else len(word) - 1 - index

    word = rotate_at(word, index - diff)
    return rotate_at(word, index + diff)

def f(word, index):
    if (index + 1) % 2 == 0:
        return rotate_at(word, int(index / 2))

    else:
        return rotate_at(word, int((index + 9) / 2))

def click(word, index):
    if word[index] == 'A':
        return a(word, index)
    elif word[index] == 'B':
        return b(word, index)
    elif word[index] == 'C':
        return c(word, index)
    elif word[index] == 'D':
        return d(word, index)
    elif word[index] == 'E':
        return e(word, index)
    elif word[index] == 'F':
        return f(word, index)



to_visit = []
visited = set()
next_lvl = []
steps = 0
complete = False

to_visit.append(input().strip())
end = input().strip()

while len(to_visit) > 0 and not complete:
    while len(to_visit) > 0 and not complete:
        state = to_visit.pop(0)

        if state == end:
            complete = True
            break

        #if state in visited: # WHY DOES THIS BREAK IT
        #    break

        visited.add(state)

        for i in range(0, 8):
            next_state = click(state, i)
            if next_state not in visited:
                next_lvl.append(next_state)

    to_visit.extend(next_lvl)
    next_lvl = []

    if not complete:
        steps += 1 
        print(steps)



print(steps)
