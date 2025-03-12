import matplotlib.pyplot as plt
import numpy as np

x = [1,5,10,15,20,25,30,35,40]
y = [8.61/8.47,8.67/2.52,8.67/1.35,8.67/1.03,8.67/0.89,8.67/0.85,8.67/0.86,8.67/0.83,8.67/0.85]

plt.figure(figsize=(10, 6))

# Plot with markers and line
plt.plot(x, y, 'o-', color='#2070b0', linewidth=2, markersize=8)

# Find the minimum value and its position
min_idx = y.index(min(y))
min_x, min_y = x[min_idx], y[min_idx]

# Add grid
plt.grid(True, linestyle='--', alpha=0.7)

# Labels and title
plt.xlabel('P value (threads)', fontsize=12)
plt.ylabel('Speed up', fontsize=12)
plt.title('Speedup of parallel vs sequential (N=45,000,000)', fontsize=14)

# Set x-axis ticks to show all values
plt.xticks(x)

# Add legend
plt.legend()


# Tighten layout
plt.tight_layout()

# Show plot
plt.show()