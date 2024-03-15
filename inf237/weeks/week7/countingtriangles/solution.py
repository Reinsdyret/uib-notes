class Point:
    def __init__(self, x, y):
        self.x = x 
        self.y = y

    def __str__(self):
        return f"({self.x}, {self.y})"

def orientation(p1, q1, p2):
    val = ((q1.y - p1.y) *
           (p2.x - q1.x) -
           (q1.x - p1.x) *
           (p2.y - q1.y))

    if val > 0:
        return 1 # Clockwise rotation
    elif val < 0:
        return -1 # Counterclockwise rotation
    else:
        return 0 # Collinear orientation

def on_segment(p, q, r):
    # Check if point q lies on line segment 'pr'
    if (q[0] <= max(p[0], r[0]) and q[0] >= min(p[0], r[0]) and
       q[1] <= max(p[1], r[1]) and q[1] >= min(p[1], r[1])):
        return True
    return False


def intersection_test(l1, l2):
    p1, q1 = l1 # p1 is start of l1 and q1 is end of l1
    p2, q2 = l2
    o1 = orientation(p1, q1, p2) # Check where start of l2 lies in terms of l1 (right, left or colinear)
    o2 = orientation(p1, q1, q2)
    o3 = orientation(p2, q2, p1)
    o4 = orientation(p2, q2, q1)

    if o1 != o2 and o3 != o4: # General case, are the start and end of each line of seperate sides of the other line
        return True

    # Special cases
    if o1 == 0 and on_segment(p1, p2, q1): # If the start of l2 is colinear with l1 and it actually is on the line it intersects
        return True

    if o2 == 0 and on_segment(p1, q2, q1):
        return True

    if o3 == 0 and on_segment(p2, p1, q2):
        return True

    if o4 == 0 and on_segment(p2, q1, q2):
        return True

    return False

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
                    #print(f"TRIANGLE with {(str(p1), str(q1))}, {(str(p2), str(q2))}, {(str(p3), str(q3))}")
                    count += 1
    print(count)
    n = int(input())
