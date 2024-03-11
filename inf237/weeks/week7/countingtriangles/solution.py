class Point:
    def __init__(self, x, y):
        self.x = x 
        self.y = y

    def __str__(self):
        return f"({self.x}, {self.y})"

def orientation(p1, q1, p2):
    return ((q1.y - p1.y) *
           (p2.x - q1.x) -
           (q1.x - p1.x) *
           (p2.y - q1.y))

def intersection_test(l1, l2):
    p1, q1 = l1
    p2, q2 = l2
    o1 = orientation(p1, q1, p2)
    o2 = orientation(p1, q1, q2)
    o3 = orientation(p2, q2, p1)
    o4 = orientation(p2, q2, q1)

    return o1 != o2 and o3 != o4 and o1 != 0 and o2 != 0 and o3 != 0 and o4 != 0


n = int(input())
while n != 0:
    lines = []
    for _ in range(n):
        x1, y1, x2, y2 = map(float, input().split())
        lines.append((Point(x1, y1), Point(x2, y2)))
    
    # Counting triangles
    count = 0
    for i in range(len(lines)):
        for j in range(i + 1, len(lines)):
            for k in range(j + 1, len(lines)):
                l1 = lines[i]
                l2 = lines[j]
                l3 = lines[k]
                if intersection_test(l1, l2) and intersection_test(l2, l3) and intersection_test(l1, l3):
                    p1, q1 = l1
                    p2, q2 = l2
                    p3, q3 = l3
                    print(f"TRIANGLE with {(str(p1), str(q1))}, {(str(p2), str(q2))}, {(str(p3), str(q3))}")
                    count += 1
    print(count)
    n = int(input())
