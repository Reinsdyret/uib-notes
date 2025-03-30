#!/usr/bin/env python3
import os
import glob
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
import argparse
from matplotlib.ticker import ScalarFormatter
import matplotlib as mpl

def plot_operator_weights(weights_file, best_history_file=None, output_dir="plots", show_plots=False):
    """
    Plot the operator weights from a CSV file.
    
    Args:
        weights_file: Path to the CSV file containing operator weights
        best_history_file: Optional path to the CSV file with solution history 
        output_dir: Directory to save plot images
        show_plots: Whether to display plots interactively
    """
    # Create output directory if it doesn't exist
    os.makedirs(output_dir, exist_ok=True)
    
    # Get the problem name from the file path
    file_name = os.path.basename(weights_file)
    problem_name = file_name.replace("_weights.csv", "")
    
    # Read the weights CSV file
    weights_data = pd.read_csv(weights_file)
    
    # Apply some smoothing for clearer visualization (optional)
    # weights_data_smooth = weights_data.copy()
    # for col in weights_data.columns[1:]:
    #     weights_data_smooth[col] = weights_data[col].rolling(window=5, min_periods=1).mean()
    
    # Get the operator names (excluding iteration column)
    operator_columns = weights_data.columns[1:]
    num_operators = len(operator_columns)
    
    # Define the actual operator names in the correct order
    operator_names = [
        "random_removal_greedy_insert",
        "worst_removal_greedy_insert",
        "route_removal_greedy_insert",
        "one_reinsert_greedy_insert",
        "shaw_removal_greedy_insert",
        "random_removal_first_feasible_insert"
    ]
    
    # Create a mapping of column indices to operator names
    # Only use as many names as we have columns
    name_mapping = {
        f"operator_{i}": name for i, name in enumerate(operator_names) 
        if i < num_operators
    }
    
    # Create a colormap with enough colors for all operators
    cmap_name = 'tab10' if num_operators <= 10 else 'tab20'
    colors = [plt.get_cmap(cmap_name)(i) for i in range(num_operators)]
    
    # Setup the plot with a white background
    # Increase width to accommodate the side legend
    plt.figure(figsize=(16, 8), facecolor='white')
    ax = plt.gca()
    ax.set_facecolor('white')
    
    # Plot weights for each operator with improved styling
    for i, op_col in enumerate(operator_columns):
        # Get the proper operator name, or use the column name if not in mapping
        display_name = name_mapping.get(op_col, op_col)
        
        plt.plot(weights_data['iteration'], weights_data[op_col], 
                 label=display_name, color=colors[i], linewidth=1.5, alpha=0.9)
    
    # Plot best solution improvement points if history file is provided
    if best_history_file:
        solution_data = pd.read_csv(best_history_file)
        best_costs = solution_data['best_cost'].values
        
        # Find iterations where best solution improves
        # Only consider significant improvements (e.g., > 0.1% improvement)
        improvements = []
        for i in range(1, len(best_costs)):
            if best_costs[i] < best_costs[i-1]:
                rel_improvement = (best_costs[i-1] - best_costs[i]) / best_costs[i-1]
                if rel_improvement > 0.001:  # 0.1% improvement threshold
                    improvements.append(i)
        
        if len(improvements) > 0:
            # Limit to at most 20 improvement markers to avoid cluttering
            if len(improvements) > 20:
                # Find the most significant improvements
                significant_improvements = []
                for i in range(1, len(best_costs)):
                    if best_costs[i] < best_costs[i-1]:
                        rel_improvement = (best_costs[i-1] - best_costs[i]) / best_costs[i-1]
                        significant_improvements.append((i, rel_improvement))
                
                significant_improvements.sort(key=lambda x: x[1], reverse=True)
                improvements = [x[0] for x in significant_improvements[:20]]
            
            # Plot vertical lines for improvements with better styling
            for improvement in improvements:
                plt.axvline(x=improvement, color='darkgray', linestyle='--', 
                           linewidth=0.8, alpha=0.5, zorder=0)
    
    # Add labels and title with better styling
    plt.xlabel('Iteration', fontsize=12, fontweight='bold')
    plt.ylabel('Weight', fontsize=12, fontweight='bold')
    plt.title(f'Operator Weights for {problem_name}', fontsize=14, fontweight='bold')
    
    # Move legend to the right side of the plot
    legend = plt.legend(loc='center left', bbox_to_anchor=(1.01, 0.5),
                      frameon=True, fontsize=10)
    legend.get_frame().set_facecolor('white')
    legend.get_frame().set_alpha(0.9)
    legend.get_frame().set_linewidth(0.5)
    
    # Y-axis limits with a bit of padding
    plt.ylim(0, max(1.0, weights_data[operator_columns].max().max() * 1.1))
    
    # Add grid with lighter styling
    plt.grid(True, linestyle='--', alpha=0.3, color='gray')
    
    # Improve tick labels
    plt.xticks(fontsize=10)
    plt.yticks(fontsize=10)
    
    # Add a box around the plot
    plt.box(True)
    
    # Save the plot with higher resolution
    output_path = os.path.join(output_dir, f"{problem_name}_weights_plot.png")
    plt.savefig(output_path, dpi=200, bbox_inches='tight', facecolor='white')
    print(f"Plot saved to {output_path}")
    
    if show_plots:
        plt.show()
    else:
        plt.close()

def save_weights_to_csv(weights_history, output_file, operator_names=None):
    """
    Save the weights history to a CSV file.
    
    Args:
        weights_history: List of lists containing weights at each iteration
        output_file: Path to the output CSV file
        operator_names: Optional list of operator names for column headers
    """
    # Create output directory if it doesn't exist
    os.makedirs(os.path.dirname(output_file), exist_ok=True)
    
    # Create a dataframe
    df = pd.DataFrame(weights_history)
    
    # Set column names
    if operator_names:
        df.columns = operator_names
    else:
        df.columns = [f'operator_{i}' for i in range(df.shape[1])]
    
    # Add iteration column
    df.insert(0, 'iteration', range(len(weights_history)))
    
    # Save to CSV
    df.to_csv(output_file, index=False)
    print(f"Weights saved to {output_file}")

def plot_all_weights(data_dir="output", output_dir="plots", show_plots=False):
    """
    Plot all weights CSV files in the given directory.
    
    Args:
        data_dir: Directory containing the weights CSV files
        output_dir: Directory to save plot images
        show_plots: Whether to display plots interactively
    """
    # Find all weights CSV files
    weights_files = glob.glob(os.path.join(data_dir, "*_weights.csv"))
    
    if not weights_files:
        print(f"No weights CSV files found in {data_dir}")
        return
    
    print(f"Found {len(weights_files)} weights files")
    
    # Plot each file
    for weights_file in weights_files:
        # Find corresponding history file if it exists
        base_name = os.path.basename(weights_file).replace("_weights.csv", "")
        history_file = os.path.join(data_dir, f"{base_name}_history.csv")
        
        history_file_arg = history_file if os.path.exists(history_file) else None
        plot_operator_weights(weights_file, history_file_arg, output_dir, show_plots)

def main():
    parser = argparse.ArgumentParser(description='Plot operator weights from CSV files')
    parser.add_argument('--data-dir', default='output', 
                        help='Directory containing weights CSV files (default: output)')
    parser.add_argument('--output-dir', default='plots', 
                        help='Directory to save plot images (default: plots)')
    parser.add_argument('--show', action='store_true', 
                        help='Show plots interactively')
    parser.add_argument('--file', 
                        help='Plot a specific weights file (optional)')
    parser.add_argument('--history-file',
                        help='Optional path to a specific solution history CSV file (use with --file)')
    
    args = parser.parse_args()
    
    if args.file:
        # Plot a specific file
        plot_operator_weights(args.file, args.history_file, args.output_dir, args.show)
    else:
        # Plot all files in the data directory
        plot_all_weights(args.data_dir, args.output_dir, args.show)

if __name__ == "__main__":
    main()