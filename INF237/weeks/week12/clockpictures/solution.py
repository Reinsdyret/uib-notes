
def calculate_differences(angles):
    sorted_angles = sorted(angles)
    differences = []
    for i in range(len(sorted_angles)):
        next_index = (i + 1) % len(sorted_angles)  # wrap around to the start
        diff = (sorted_angles[next_index] - sorted_angles[i] + 360000) % 360000
        differences.append(diff)
    return differences

def least_rotation(s) -> int:
    """Booth's lexicographically minimal string rotation algorithm."""
    # Taken from: https://en.wikipedia.org/wiki/Lexicographically_minimal_string_rotation 
    # Returns the minimal lexicographical rotation index, this is so we can compare clock hands with the same benchmark
    n = len(s)
    f = [-1] * (2 * n)
    k = 0
    for j in range(1, 2 * n):
        i = f[j - k - 1]
        while i != -1 and s[j % n] != s[(k + i + 1) % n]:
            if s[j % n] < s[(k + i + 1) % n]:
                k = j - i - 1
            i = f[i]
        if i == -1 and s[j % n] != s[(k + i + 1) % n]:
            if s[j % n] < s[(k + i + 1) % n]:
                k = j
            f[j - k] = -1
        else:
            f[j - k] = i + 1
    return k

n = int(input())
clock1 = map(int, input().split())
clock2 = map(int, input().split())

diff1 = calculate_differences(clock1)
diff2 = calculate_differences(clock2)

index1 = least_rotation(diff1)
index2 = least_rotation(diff2)
i1 = index1
i2 = index2
flag = True

while index1 < len(diff1) + i1 and index2 < len(diff2) + i2:
    if diff1[index1 % len(diff1)] != diff2[index2 % len(diff2)]: 
        flag = False
        break
    index1 += 1
    index2 += 1
    
if flag:
    print("possible")
else:
    print("impossible")
