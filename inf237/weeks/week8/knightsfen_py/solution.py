import sys
from collections import deque
from functools import lru_cache
    
#@lru_cache(None)
def next_configurations(board):
    empty_square = (-1, -1)
    for y in range(5):
        if empty_square != (-1, -1):
            break
        for x in range(5):
            if board[y * 5 + x] == ' ':
                empty_square = (x, y)
                break
    
    x,y = empty_square
    cases = [
        (x + 1, y + 2),
        (x + 2, y + 1),
        (x + 2, y - 1),
        (x + 1, y - 2),
        (x - 1, y - 2),
        (x - 2, y - 1),
        (x - 2, y + 1),
        (x - 1, y + 2)
    ]

    for (horsex, horsey) in cases:
        if horsex > 4 or horsey > 4 or horsex < 0 or horsey < 0:
            continue
        new_board = ""
        for c in range(len(board)):
            if c == y * 5 + x:
                new_board += board[horsey * 5 + horsex]
            elif c == horsey * 5 + horsex:
                new_board += ' '
            else:
                new_board += board[c]
        yield new_board


def is_finish_conf(board, goal):
    return board == goal

def bfs_board_min_depth(board, goal):
    queue = deque([(board, 0)])
    visited = set([board])

    while queue:
        curr_board, depth = queue.popleft()
        
        if is_finish_conf(curr_board, goal):
            return depth

        if depth == 10:
            return -1
    
        for next_board in next_configurations(curr_board):
            next_hash = next_board
            if next_hash not in visited:
                visited.add(next_hash)
                queue.append((next_board, depth + 1))

    return -1


n = int(sys.stdin.readline().strip())
if n <= 0:
    print('tee' + 76 * 7)
for case in range(n):
    b = []
    for row in range(5):
        a = sys.stdin.readline().strip()
        if len(a) != 5:
            a += ' '
        b.append(a)
    
    board = ''.join(b)
    if len(board) != 25:
        print('poo' + 76 * 7)
        #print(len(board))

    goal = "111110111100 110000100000"
    
    res = bfs_board_min_depth(goal, board)
    if res == -1:
        sys.stdout.write("Unsolvable in less than 11 move(s).\n")
    else:
        sys.stdout.write(f"Solvable in {res} move(s).\n")
