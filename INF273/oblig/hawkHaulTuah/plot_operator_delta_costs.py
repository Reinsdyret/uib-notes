#!/usr/bin/env python3
import os
import glob
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
import argparse
from matplotlib.ticker import ScalarFormatter
import matplotlib as mpl

def plot_delta_costs(delta_costs_file, output_dir="plots", show_plots=False):
    """
    Plot the operator delta costs from a CSV file.
    
    Args:
        delta_costs_file: Path to the CSV file containing operator delta costs
        output_dir: Directory to save plot images
        show_plots: Whether to display plots interactively
    """
    # Create output directory if it doesn't exist
    os.makedirs(output_dir, exist_ok=True)
    
    # Get the problem name from the file path
    file_name = os.path.basename(delta_costs_file)
    problem_name = file_name.replace("_delta_costs.csv", "")
    
    # Read the delta costs CSV file
    delta_data = pd.read_csv(delta_costs_file)
    
    # Get the operator names (excluding iteration column)
    operator_columns = delta_data.columns[1:]
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
    plt.figure(figsize=(16, 8), facecolor='white')
    ax = plt.gca()
    ax.set_facecolor('white')
    
    # Plot delta costs for each operator with improved styling
    for i, op_col in enumerate(operator_columns):
        # Get the proper operator name, or use the column name if not in mapping
        display_name = name_mapping.get(op_col, op_col)
        
        # Filter out zeros (iterations where the operator wasn't used)
        filtered_data = delta_data[['iteration', op_col]].copy()
        filtered_data = filtered_data[filtered_data[op_col] != 0]
        
        if not filtered_data.empty:
            plt.scatter(
                filtered_data['iteration'], 
                filtered_data[op_col], 
                label=display_name, 
                color=colors[i], 
                alpha=0.6,
                s=25  # Marker size
            )
    
    # Add labels and title with better styling
    plt.xlabel('Iteration', fontsize=12, fontweight='bold')
    plt.ylabel('Delta Cost', fontsize=12, fontweight='bold')
    plt.title(f'Operator Delta Costs for {problem_name}', fontsize=14, fontweight='bold')
    
    # Move legend to the right side of the plot
    legend = plt.legend(loc='center left', bbox_to_anchor=(1.01, 0.5),
                     frameon=True, fontsize=10)
    legend.get_frame().set_facecolor('white')
    legend.get_frame().set_alpha(0.9)
    legend.get_frame().set_linewidth(0.5)
    
    # Add grid with lighter styling
    #plt.grid(True, linestyle='--', alpha=0.3, color='gray')
    
    # Improve tick labels
    plt.xticks(fontsize=10)
    plt.yticks(fontsize=10)
    
    # Add a horizontal line at y=0 to show improvement boundary
    plt.axhline(y=0, color='black', linestyle='-', alpha=0.3)
    
    # Add annotations for improvements vs. non-improvements
    plt.annotate('Improving Moves', xy=(0.02, 0.05), xycoords='axes fraction', 
                fontsize=10, fontweight='bold', color='green')
    plt.annotate('Non-Improving Moves', xy=(0.02, 0.95), xycoords='axes fraction', 
                fontsize=10, fontweight='bold', color='red')
    
    # Add a box around the plot
    plt.box(True)
    
    # Save the plot with higher resolution
    output_path = os.path.join(output_dir, f"{problem_name}_delta_costs_plot.png")
    plt.savefig(output_path, dpi=200, bbox_inches='tight', facecolor='white')
    print(f"Plot saved to {output_path}")
    
    if show_plots:
        plt.show()
    else:
        plt.close()

def plot_all_delta_costs(data_dir="output", output_dir="plots", show_plots=False):
    """
    Plot all delta costs CSV files in the given directory.
    
    Args:
        data_dir: Directory containing the delta costs CSV files
        output_dir: Directory to save plot images
        show_plots: Whether to display plots interactively
    """
    # Find all delta costs CSV files
    delta_costs_files = glob.glob(os.path.join(data_dir, "*_delta_costs.csv"))
    
    if not delta_costs_files:
        print(f"No delta costs CSV files found in {data_dir}")
        return
    
    print(f"Found {len(delta_costs_files)} delta costs files")
    
    # Plot each file
    for delta_costs_file in delta_costs_files:
        plot_delta_costs(delta_costs_file, output_dir, show_plots)

def main():
    parser = argparse.ArgumentParser(description='Plot operator delta costs from CSV files')
    parser.add_argument('--data-dir', default='output', 
                        help='Directory containing delta costs CSV files (default: output)')
    parser.add_argument('--output-dir', default='plots', 
                        help='Directory to save plot images (default: plots)')
    parser.add_argument('--show', action='store_true', 
                        help='Show plots interactively')
    parser.add_argument('--file', 
                        help='Plot a specific delta costs file (optional)')
    
    args = parser.parse_args()
    
    if args.file:
        # Plot a specific file
        plot_delta_costs(args.file, args.output_dir, args.show)
    else:
        # Plot all files in the data directory
        plot_all_delta_costs(args.data_dir, args.output_dir, args.show)

if __name__ == "__main__":
    main()
