
def is_valid(s, pattern):
    if len(pattern) == 1:
        if pattern[0] == '*':
            return True
        return s[0] == pattern[0] and len(s) == 1

    if pattern[0] == '*':
        pattern = pattern[1:]

        if '*' not in pattern:
            return s[-len(pattern):] == pattern

        while s[0] != pattern[0]:
            if len(s) == 1:
                return False
            s = s[1:]
    
    if len(pattern) == 1:
        if pattern[0] == '*':
            return True
        return pattern[0] == s[0] and len(s) == 1
    
    return is_valid(s[1:], pattern[1:])

pattern = input().strip()
n = int(input().strip())

for _ in range(n):
    s = input().strip()
    if is_valid(s, pattern):
        print(s)
