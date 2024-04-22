
def percentage_complete(henchmen, need_to_bribe, money, memo):
    if need_to_bribe == 0:
        return 1
    if len(henchmen) == 0 or money == 0:
        return 0

    new_prob = 0
    for cost, probability in henchmen:
        if cost <= money:
            new_prob = max(new_prob, 
                           percentage_complete(henchmen - {(cost, probability)}, need_to_bribe - 1, money - cost, memo) * percentage + 
                           percentage_complete(henchmen - {(cost, probability)}, need_to_bribe, money - cost, memo) * (1 - percentage))

            if (tuple(henchmen), need_to_bribe, money) in memo:
                new_prob = max(memo[(tuple(henchmen), need_to_bribe, money)], new_prob)
                memo[(tuple(henchmen), need_to_bribe, money)] = new_prob

    return new_prob

test_cases = int(input())

for case in range(test_cases):
    number_henchmen, need_to_bribe, money = map(int, input().split())

    henchmen = set()
    for _ in range(number_henchmen):
        cost, percentage = map(int, input().split())
        henchmen.add((cost, percentage * 0.01))

    dp = {}
    for i in range(number_henchmen + 1):
        dp[i] = {}
        for j in range(money + 1):
            dp[i][j] = 0

    #dp[0][0] = 1


    """for (cost, percentage) in henchmen:
        for n_henchmen in range(number_henchmen, 0, -1):
            for curr_money in range(money, -1, -1):
                if curr_money >= cost:
                    dp[n_henchmen][curr_money] = max(dp[n_henchmen][curr_money],
                                                     dp[n_henchmen - 1][curr_money - cost] * percentage + 
                                                     dp[n_henchmen][curr_money - cost] * (1 - percentage))"""

    print(percentage_complete(henchmen, need_to_bribe, money, {}))


