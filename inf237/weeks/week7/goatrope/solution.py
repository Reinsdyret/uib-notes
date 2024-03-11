import math
from decimal import *
x, y, x1, y1, x2, y2 = map(Decimal, input().split())


in_x = x1 <= x <= x2
in_y = y1 <= y <= y2
greater_y = y >= y2
greater_x = x >= x2


if x < x1: # On the left
    dx = x1 - x
elif x > x2: # On the right
    dx = x - x2
else:
    dx = 0

if y < y1: # Under square
    dy = y1 - y
elif y > y2: # Over square
    dy = y2 - y
else: 
    dy = 0

print(math.sqrt(dx ** 2 + dy ** 2))