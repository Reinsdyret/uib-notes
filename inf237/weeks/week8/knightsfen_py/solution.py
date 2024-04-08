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

def bfs_board_min_depth(goal):
    queue = deque([(goal, 0)])
    visited = {goal: 0}

    while queue:
        curr_board, depth = queue.popleft()
        
        if visited[curr_board] == 10:
            return visited
    
        for next_board in next_configurations(curr_board):
            if next_board not in visited:
                visited[next_board] = depth + 1
                queue.append((next_board, depth + 1))

    return visited


#n = int(sys.stdin.readline().strip())
goal = "111110111100 110000100000"
boards = bfs_board_min_depth(goal)
#print(boards)
with open('new1.py', 'w') as f:
    
    f.write(f"""
n = int(input())
boards = {str(boards)}
for case in range(n):
    b = []
    for row in range(5):
        a = input().strip()
        if len(a) != 5:
            a += ' '
        b.append(a)
    
    board = ''.join(b)
    if board in boards:
        print(f"Solvable in boards[board] move(s).")
    else:
        print("Unsolvable in less than 11 move(s).")
    """)
    
