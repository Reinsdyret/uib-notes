def percentage_complete(henchmen, need_to_bribe, money, memo):
    if need_to_bribe == 0:
        return 1
    if len(henchmen) == 0 or money == 0:
        return 0
    
    # Check if the result for the current state is already memoized
    if (tuple(henchmen), need_to_bribe, money) in memo:
        return memo[(tuple(henchmen), need_to_bribe, money)]
    
    best_prob = 0
    for cost, percentage in henchmen:
        if cost <= money:
            prob_success = percentage_complete(henchmen - {(cost, percentage)}, need_to_bribe - 1, money - cost, memo) * percentage
            prob_failure = percentage_complete(henchmen - {(cost, percentage)}, need_to_bribe, money - cost, memo) * (1 - percentage)
            best_prob = max(best_prob, prob_success + prob_failure)
    
    # Store the best probability found for the current state in the memoization table
    memo[(tuple(henchmen), need_to_bribe, money)] = best_prob
    
    return best_prob

test_cases = int(input())

for case in range(test_cases):
    number_henchmen, need_to_bribe, money = map(int, input().split())

    henchmen = set()
    for _ in range(number_henchmen):
        cost, percentage = map(int, input().split())
        henchmen.add((cost, percentage * 0.01))

    print(percentage_complete(henchmen, need_to_bribe, money, {}))


