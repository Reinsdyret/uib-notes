import matplotlib.pyplot as plt
import numpy as np

x = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]
y = [25.45,13.34,10.53,9.71,15.34,12.24,11.56,10.59,9.79,9.45,8.67,8.95,9.57,10.15,10.28,9.07]

plt.figure(figsize=(10, 6))

# Plot with markers and line
plt.plot(x, y, 'o-', color='#2070b0', linewidth=2, markersize=8)

# Find the minimum value and its position
min_idx = y.index(min(y))
min_x, min_y = x[min_idx], y[min_idx]

# Highlight the minimum point
plt.plot([min_x], [min_y], 'ro', markersize=10, label=f'Minimum: {min_y:.2f}s at B={min_x}')

# Add grid
plt.grid(True, linestyle='--', alpha=0.7)

# Labels and title
plt.xlabel('B Value (bits)', fontsize=12)
plt.ylabel('Execution Time (seconds)', fontsize=12)
plt.title('Effect of B Value on Radix Sort Execution Time (N=45,000,000)', fontsize=14)

# Set x-axis ticks to show all values
plt.xticks(x)

# Add legend
plt.legend()

# Add annotations for interesting points
plt.annotate(f'B={min_x}, Time={min_y:.2f}s', 
            xy=(min_x, min_y), 
            xytext=(min_x+1, min_y-1),
            arrowprops=dict(facecolor='black', shrink=0.05, width=1.5),
            fontsize=10)

# Tighten layout
plt.tight_layout()

# Show plot
plt.show()