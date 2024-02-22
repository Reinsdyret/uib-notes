width, height = map(int, input().strip().split())

key = input().strip()

grid = []

for _ in range(height):
    row = list(map(int, input().strip()))
    grid.append(row)

grid.reverse()

dp = {}

for y in range(height):
    dp[y] = {}
    for x in range(width):
        dp[y][x] = {}
        for possiblekey in [key[i:] for i in range(len(key), -1, -1)]:
            if x == width - 1 and y == height - 1:
                dp[y][x][possiblekey] = grid[y][x]
            else:
                dp[y][x][possiblekey] = float('inf')


for possiblekey in [key[i:] for i in range(len(key), -1, -1)]:
    for x in range(width - 1, -1, -1):
        for y in range(height - 1, -1, -1):

            if x == width - 1 and y == height - 1:
                continue

            hopDigit = int(possiblekey[0]) if len(possiblekey) >= 1 else 0


            moves = [
                (1,             0, possiblekey),
                (0,             1, possiblekey),
                (hopDigit + 1,  0, possiblekey[1:]),
                (0,  hopDigit + 1, possiblekey[1:])
            ]


            for (dx, dy, key1) in moves:
                if (x + dx <= width - 1)  and (y + dy <= height - 1):
                    dp[y][x][possiblekey] = min(dp[y][x][possiblekey], dp[y + dy][x + dx][key1] + grid[y][x]) 


#for y in range(height - 1, -1, -1):
#    print()
#    for x in range(width - 1, -1, -1):
#        print(dp[y][x])
print(dp[0][0][key])
#print(dp[height - 1][width - 1])
                
            