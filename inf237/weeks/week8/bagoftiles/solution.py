from math import comb

def countCombinations(tiles, k, targetSum):
    count = 0

    # Generate all combinations of k tiles from the given tiles
    def generateCombinations(tiles, k):
        # This is a helper function that should generate all combinations
        # of k tiles from the given list of tiles.
        # It should yield each combination one by one.
        

    # Iterate over each combination of k tiles
    for combination in generateCombinations(tiles, k):

        # If the sum equals the target sum, increment the count
        if sum(combination) == targetSum:
            count += 1

    return count


games = int(input())

for game in range(games):
    n_tiles = int(input())
    tiles = list(map(int, input().split()))
    n, t = map(int, input().split())

    #print(, comb(n_tiles, n))
    print(f"Game {game + 1} -- {count(tiles, n, t)} : {comb(n_tiles, n) - count(tiles, n, t)}")

