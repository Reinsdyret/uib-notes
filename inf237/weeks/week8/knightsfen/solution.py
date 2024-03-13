import copy
class Board:
    def __init__(self, board):
        self.board = board
    
    def next_configurations(self):
        empty_square = (-1, -1)
        for y, row in enumerate(self.board):
            if empty_square != (-1, -1):
                break
            for x, square in enumerate(row):
                if square == ' ':
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
            new_board = copy.deepcopy(self.board)
            new_board[y][x] = new_board[horsey][horsex]
            new_board[horsey][horsex] = ' '
            yield Board(new_board)
        
        return

    def is_finish_conf(self):
        goal = [
            ['1','1','1','1','1'],
            ['0','1','1','1','1'],
            ['0','0',' ','1','1'],
            ['0','0','0','0','1'],
            ['0','0','0','0','0']
        ]
        for row in range(5):
            for col in range(5):
                if goal[row][col] != self.board[row][col]:
                    return False
        return True
    
    def __str__(self):
        return '\n'.join([(' '.join(map(str, row))) for row in self.board])

def dfs_board_min_depth(depth, board, visited):
    if board in visited:
        return -1
    if depth >= 11:
        #print("Reached max depth")
        visited.add(board)
        return -1

    if board.is_finish_conf():
        return depth
    
    for conf in board.next_configurations():
        #print(str(board))
        #print(str(conf))
        d = dfs_board_min_depth(depth + 1, conf, visited)
        if d != -1 and d < 11:
            return d
    visited.add(board)
    return -1


n = int(input())
for case in range(n):
    b = []
    for row in range(5):
        b.append(list(input()))

    board = Board(b)

    #print(f"{board.is_finish_conf() = }")
    
    res = dfs_board_min_depth(0, board, set())
    if res == -1:
        print("Unsolvable in less than 11 move(s).")
    else:
        print(f"Solvable in {res} move(s).")
