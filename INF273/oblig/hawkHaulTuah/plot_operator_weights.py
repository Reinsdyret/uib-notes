import matplotlib.pyplot as plt

x = [a for a in range(1000, 24900)]
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
y1 = []
y2 = []
with open('output.txt', 'r', encoding='utf-16')  as f:
    line = f.readline()
    vals = line.strip()[:-1].split(',')
    y1 = list(map(int, vals))[1000:]
    line = f.readline()
    vals = line.strip()[:-1].split(',')
    y2 = list(map(int, vals))[1000:]



print(min(y1))
plt.plot(x, y1, color='blue', label='Best')
plt.plot(x, y2, color='red', label='Inc')
# Add labels and a legend
plt.xlabel('X-axis')
plt.ylabel('Y-axis')
# plt.title('Three Lines Plot')
plt.legend()

# Add a grid (optional)
plt.grid(True, linestyle='--', alpha=0.7)
plt.yscale('log')

# Show the plot
plt.show()
