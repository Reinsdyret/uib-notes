with open("numbers.rule","a") as rule:
    for num1 in range(0,10):
        for num2 in range(0,10):
            for num3 in range(0,10):
                for num4 in range(0,10):
                    rule.write(f"${num1} ${num2} ${num3} ${num4}\n")