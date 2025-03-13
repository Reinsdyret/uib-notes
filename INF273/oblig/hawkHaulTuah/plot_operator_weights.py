import matplotlib.pyplot as plt

x = [a for a in range(25000)]
# y1 = []
# y2 = []
# y3 = []
# with open('output.txt', 'r',encoding = "utf-16") as f:
#     for line in f.readlines():
#         a,b,c = map(float, map(lambda i: i.strip(), line.strip().split(',')))
#         y1.append(a)
#         y2.append(b)
#         y3.append(c)
#
# # Create a figure and axis
plt.figure(figsize=(10, 6))
#
# # Plot the three lines
# plt.plot(x, y1, label='Line 1', color='blue')
# plt.plot(x, y2, label='Line 2', color='red')
# plt.plot(x, y3, label='Line 3', color='green')
y = []
with open('output.txt', 'r')  as f:
    line = f.readline()
    y.extend(map(int, line.strip()[:-1].split(',')))

print(min(y))

plt.plot(x,y)
# Add labels and a legend
plt.xlabel('X-axis')
plt.ylabel('Y-axis')
# plt.title('Three Lines Plot')
plt.legend()

# Add a grid (optional)
plt.grid(True, linestyle='--', alpha=0.7)

# Show the plot
plt.show()
