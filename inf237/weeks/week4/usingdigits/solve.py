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
        for possiblekey in [key[0:i] for i in range(len(key), -1, -1)]:
            if x == width - 1 and y == height - 1:
                dp[y][x][possiblekey] = grid[y][x]
            else:
                dp[y][x][possiblekey] = 1000000000

print(dp)

#print(key)
for x in range(width - 1, -1, -1):
    for y in range(height - 1, -1, -1):
        for possiblekey in [key[0:i] for i in range(len(key), 0, -1)]:
            hopDigit = int(possiblekey[0])
            hopX = x + hopDigit
            hopY = y + hopDigit
            nextX = x + 1 if x <= width - 2 else x
            nextY = y + 1 if y <= height - 2 else y
            if hopX < width and hopY < height:
                jumpY = min(dp[nextY][x][possiblekey], dp[hopY][x][possiblekey[1:]])
                jumpX = min(dp[y][nextX][possiblekey], dp[y][hopX][possiblekey[1:]])
                print(jumpX, jumpY)
                dp[y][x][possiblekey] = min(dp[y][x][possiblekey], min(jumpX, jumpY) + grid[y][x])
            else:
                jumpY = dp[nextY][x][possiblekey]
                jumpX = dp[y][nextX][possiblekey]
                dp[y][x][possiblekey] = min(dp[y][x][possiblekey], min(jumpX, jumpY) + grid[y][x])

print((dp[0][0]))
print(dp[height - 1][width - 2])
                
            