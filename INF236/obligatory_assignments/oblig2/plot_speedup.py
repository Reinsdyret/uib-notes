import matplotlib.pyplot as plt
import numpy as np

# Input data (hardcoded from your results)
# Thread configurations
threads = [1,10,20,25,30,35,40,45,50]

# Number of graphs and thread configurations
num_graphs = 4
num_thread_configs = 9

# Sequential BFS times for each graph
TimeSequentialMIS = [0.733788,0.531854 ,1.240490 ,0.132739 ,];
TimeParallelLUBYS = [2.450543, 0.582116, 0.566319, 0.623455, 0.656037, 0.641862, 0.651390, 0.713429, 0.641149, 1.950494, 0.487295, 0.498011, 0.544064, 0.570197, 0.613234, 0.597369, 0.618892, 0.617608, 4.078363, 0.856273, 0.737787, 0.799110, 0.789762, 0.832289, 0.870314, 0.896534, 0.905202, 0.483776, 0.097954, 0.092129, 0.082488, 0.083026, 0.088210, 0.107110, 0.113524, 0.117235, ];


# Graph names
graph_names = ["road_usa", "delaunay_n24", "hugebubbles-00020", "rgg_n_2_24_s0"] 

# Create plots for each graph
for i in range(num_graphs):
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(15, 6))
    
    # Extract data for this graph (stride through the flattened arrays)
    pbfs_times_for_graph = TimeParallelLUBYS[i*num_thread_configs:(i+1)*num_thread_configs]
    
    # Calculate speedups
    pbfs_speedup = [TimeSequentialMIS[i] / time for time in pbfs_times_for_graph]
    
    # Calculate relative speedups (relative to 1st thread configuration)
    pbfs_rel_speedup = [pbfs_times_for_graph[0] / time for time in pbfs_times_for_graph]
    
    # Absolute Speedup plot
    ax1.plot(threads, pbfs_speedup, 'o-', label='LUBYS', color='blue', linewidth=2)
    ax1.set_title(f'Absolute Speedup - {graph_names[i]}')
    ax1.set_xlabel('Number of Threads')
    ax1.set_ylabel('Speedup vs Sequential')
    ax1.grid(True, linestyle='--', alpha=0.7)
    ax1.set_xticks(threads)
    ax1.set_xticklabels(threads)
    ax1.axhline(y=1, color='gray', linestyle='--', alpha=0.7)
    ax1.legend()
    
    # Relative Speedup plot
    ax2.plot(threads, pbfs_rel_speedup, 'o-', label='LUBYS', color='blue', linewidth=2)
    ax2.set_title(f'Relative Speedup - {graph_names[i]}')
    ax2.set_xlabel('Number of Threads')
    ax2.set_ylabel('Speedup vs 1-Thread Parallel')
    ax2.grid(True, linestyle='--', alpha=0.7)
    ax2.set_xticks(threads)
    ax2.set_xticklabels(threads)
    ax2.axhline(y=1, color='gray', linestyle='--', alpha=0.7)
    ax2.legend()
    
    plt.tight_layout()
    plt.savefig(f'{graph_names[i]}_speedup.png', dpi=300)
    plt.show()
