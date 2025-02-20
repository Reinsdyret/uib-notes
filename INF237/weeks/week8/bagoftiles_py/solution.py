from math import comb

games = int(input())

for game in range(games):
    n_tiles = int(input())
    tiles = list(map(int, input().split()))
    n, t = map(int, input().split())
    dp = {}
    for i in range(n+1):
        dp[i] = {}
        for j in range(t+1):
            dp[i][j] = 0
    dp[0][0] = 1


    for i, tile in enumerate(tiles):
        for n_tile in range(n, 0, -1):
            for target in range(t, -1, -1):
                if target >= tile:
                    dp[n_tile][target] += dp[n_tile - 1][target-tile]

    number_wins = dp[n][t]
    number_lose = comb(n_tiles,n) - number_wins

    #print(, comb(n_tiles, n))
    print(f"Game {game + 1} -- {number_wins} : {number_lose}")
