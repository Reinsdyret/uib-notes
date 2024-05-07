import math
from decimal import Decimal

def convex_hull(points):
    # Sort the points
    points = sorted(points)

    def cross_product(o, a, b):
        # Returns positive value if a to b makes a counter clockwise turn relative to o
        # Returns a negative value if a t b makes a clockwise turn relative to o
        # Returns zero if the poitns o a b are collinear
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])

    # Lower hull
    lower = []
    for p in points:
        while len(lower) >= 2 and cross_product(lower[-2], lower[-1], p) <= 0:
            lower.pop()
        lower.append(p)

    # Upper hull
    upper = []
    for p in reversed(points):
        while len(upper) >= 2 and cross_product(upper[-2], upper[-1], p) <= 0:
            upper.pop()
        upper.append(p)

    return lower[:-1] + upper[:-1]



n = int(input())

points = []
for _ in range(n):
    a, b = map(Decimal, input().split())
    points.append((a,b))

points = convex_hull(points)

max_dist = 0

for i in range(len(points)):
    for j in range(len(points)):
        x1, y1 = points[i]
        x2, y2 = points[j]

        dist = (x2 - x1) ** 2 + (y2 - y1) ** 2

        if dist > max_dist:
            max_dist = dist

print(math.sqrt(max_dist))