import matplotlib.pyplot as plt
import numpy as np

# Input data (hardcoded from your results)
# Thread configurations
threads = [1,10,20,25,30,35,40,45,50]

# Number of graphs and thread configurations
num_graphs = 4
num_thread_configs = 9

# Sequential BFS times for each graph
TimeSequentialBFS = [1.584881, 1.368865, 2.813991, 1.049931]

# PBFS times (flattened array: all thread configs for graph 1, then all for graph 2, etc.)
TimeParallelBFS = [1.604629, 0.277390, 0.214463, 0.251700, 0.238942, 0.343094, 0.347714, 0.390765, 0.362409, 1.382842, 0.238847, 0.158142, 0.146521, 0.154668, 0.149344, 0.146378, 0.152951, 0.160529, 2.776183, 0.445595, 0.338240, 0.344169, 0.367489, 0.385952, 0.433809, 0.448165, 0.442961, 1.038403, 0.156478, 0.136477, 0.148004, 0.165943, 0.176832, 0.191224, 0.194180, 0.194764]

# ABFS times (same format as PBFS)
TimeAParallelBFS = [1.598266, 0.324489, 0.305016, 0.335243, 0.323191, 0.340788, 0.363755, 0.397452, 0.359143, 1.375951, 0.251043, 0.204979, 0.201393, 0.183606, 0.191885, 0.191469, 0.194692, 0.205614, 2.754033, 0.534713, 0.394210, 0.371057, 0.398509, 0.370210, 0.419829, 0.448694, 0.423002, 1.033434, 0.179274, 0.143367, 0.150642, 0.159709, 0.178846, 0.177480, 0.189029, 0.194130]

# Graph names
graph_names = ["road_usa", "delaunay_n24", "hugebubbles-00020", "rgg_n_2_24_s0"] 

# Create plots for each graph
for i in range(num_graphs):
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(15, 6))
    
    # Extract data for this graph (stride through the flattened arrays)
    pbfs_times_for_graph = TimeParallelBFS[i*num_thread_configs:(i+1)*num_thread_configs]
    abfs_times_for_graph = TimeAParallelBFS[i*num_thread_configs:(i+1)*num_thread_configs]
    
    # Calculate speedups
    pbfs_speedup = [TimeSequentialBFS[i] / time for time in pbfs_times_for_graph]
    abfs_speedup = [TimeSequentialBFS[i] / time for time in abfs_times_for_graph]
    
    # Calculate relative speedups (relative to 1st thread configuration)
    pbfs_rel_speedup = [pbfs_times_for_graph[0] / time for time in pbfs_times_for_graph]
    abfs_rel_speedup = [abfs_times_for_graph[0] / time for time in abfs_times_for_graph]
    
    # Absolute Speedup plot
    ax1.plot(threads, pbfs_speedup, 'o-', label='PBFS', color='blue', linewidth=2)
    ax1.plot(threads, abfs_speedup, 's-', label='ABFS', color='red', linewidth=2)
    ax1.set_title(f'Absolute Speedup - {graph_names[i]}')
    ax1.set_xlabel('Number of Threads')
    ax1.set_ylabel('Speedup vs Sequential')
    ax1.grid(True, linestyle='--', alpha=0.7)
    ax1.set_xticks(threads)
    ax1.set_xticklabels(threads)
    ax1.axhline(y=1, color='gray', linestyle='--', alpha=0.7)
    ax1.legend()
    
    # Relative Speedup plot
    ax2.plot(threads, pbfs_rel_speedup, 'o-', label='PBFS', color='blue', linewidth=2)
    ax2.plot(threads, abfs_rel_speedup, 's-', label='ABFS', color='red', linewidth=2)
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
