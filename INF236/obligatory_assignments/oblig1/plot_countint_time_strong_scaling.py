import matplotlib.pyplot as plt
import numpy as np

x = [1,5,10,15,20,25,30,35,40]
y = [0.84, 0.28, 0.20,0.21,0.24,0.26,0.28,0.288,0.30]

plt.figure(figsize=(10, 6))

# Plot with markers and line
plt.plot(x, y, 'o-', color='#2070b0', linewidth=2, markersize=8)

# Find the minimum value and its position
min_idx = y.index(min(y))
min_x, min_y = x[min_idx], y[min_idx]

# Highlight the minimum point
plt.plot([min_x], [min_y], 'ro', markersize=10, label=f'Minimum: {min_y:.2f}s at P={min_x}')

# Add grid
plt.grid(True, linestyle='--', alpha=0.7)

# Labels and title
plt.xlabel('P value (threads)', fontsize=12)
plt.ylabel('Counting Time (seconds)', fontsize=12)
plt.title('Effect of P on Parallel Radix Sort Counting Time (N=45,000,000)', fontsize=14)

# Set x-axis ticks to show all values
plt.xticks(x)

# Add legend
plt.legend()

# Add annotations for interesting points
plt.annotate(f'P={min_x}, Time={min_y:.2f}s', 
            xy=(min_x, min_y), 
            xytext=(min_x+1, min_y-1),
            arrowprops=dict(facecolor='black', shrink=0.05, width=1.5),
            fontsize=10)

# Tighten layout
plt.tight_layout()

# Show plot
plt.show()