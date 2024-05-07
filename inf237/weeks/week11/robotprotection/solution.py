

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


def hull_area(hull_points):
    # Shoelace formula
    n = len(hull_points)
    area = 0
    for i in range(n):
        x1, y1 = hull_points[i]
        x2, y2 = hull_points[(i + 1) % n]  # This wraps
        area += x1 * y2 - y1 * x2
    return abs(area) / 2.0


n = int(input())

while n != 0:
    points = []
    for _ in range(n):
        a, b = map(int, input().split())
        points.append((a,b))

    points = convex_hull(points))

    n = int(input())