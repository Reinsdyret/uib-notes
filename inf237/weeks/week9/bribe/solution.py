
def percentage_complete(henchmen, bribed, need_to_bribe, money):
    if need_to_bribe == 0:
        return 1
    if len(henchmen) == 0:
        return 0
        


test_cases = int(input())

for case in range(test_cases):
    number_henchmen, need_to_bribe, money = map(int, input().split())

    henchmen = []
    for _ in range(number_henchmen):
        cost, percentage = map(int, input().split())
        henchmen.append((cost, percentage * 0.01))


