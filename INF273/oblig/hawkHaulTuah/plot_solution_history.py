#!/usr/bin/env python3
import os
import glob
import pandas as pd
import matplotlib.pyplot as plt
import argparse
from matplotlib.ticker import ScalarFormatter

def plot_history_file(file_path, output_dir="plots", show_plots=False):
    """
    Plot the best and incumbent costs from a history CSV file.
    
    Args:
        file_path: Path to the CSV file
        output_dir: Directory to save plot images
        show_plots: Whether to display plots interactively
    """
    # Create output directory if it doesn't exist
    os.makedirs(output_dir, exist_ok=True)
    
    # Get the problem name from the file path
    file_name = os.path.basename(file_path)
    problem_name = file_name.replace("_history.csv", "")
    
    # Read the CSV file
    data = pd.read_csv(file_path)
    
    # Create the plot
    plt.figure(figsize=(12, 8))
    
    # Plot best cost
    plt.plot(data['iteration'], data['best_cost'], 
             label='Best Solution', color='blue', linewidth=2)
    
    # Plot incumbent cost
    plt.plot(data['iteration'], data['incumbent_cost'], 
             label='Incumbent Solution', color='red', alpha=0.7, linewidth=1)
    
    # Add labels and title
    plt.xlabel('Iteration')
    plt.ylabel('Cost')
    plt.title(f'Solution Progress for {problem_name}')
    plt.legend()
    
    # Set y-axis to scientific notation for large numbers
    plt.gca().yaxis.set_major_formatter(ScalarFormatter(useMathText=True))
    plt.ticklabel_format(style='sci', axis='y', scilimits=(0,0))
    
    # Add grid
    plt.grid(True, linestyle='--', alpha=0.7)
    
    # Save the plot
    output_path = os.path.join(output_dir, f"{problem_name}_plot.png")
    plt.savefig(output_path, dpi=150, bbox_inches='tight')
    print(f"Plot saved to {output_path}")
    
    if show_plots:
        plt.show()
    else:
        plt.close()

def plot_all_histories(data_dir="output", output_dir="plots", show_plots=False):
    """
    Plot all history CSV files in the given directory.
    
    Args:
        data_dir: Directory containing the history CSV files
        output_dir: Directory to save plot images
        show_plots: Whether to display plots interactively
    """
    # Find all history CSV files
    history_files = glob.glob(os.path.join(data_dir, "*_history.csv"))
    
    if not history_files:
        print(f"No history CSV files found in {data_dir}")
        return
    
    print(f"Found {len(history_files)} history files")
    
    # Plot each file
    for file_path in history_files:
        plot_history_file(file_path, output_dir, show_plots)

def main():
    parser = argparse.ArgumentParser(description='Plot solution history from CSV files')
    parser.add_argument('--data-dir', default='output', 
                        help='Directory containing history CSV files (default: output)')
    parser.add_argument('--output-dir', default='plots', 
                        help='Directory to save plot images (default: plots)')
    parser.add_argument('--show', action='store_true', 
                        help='Show plots interactively')
    parser.add_argument('--file', 
                        help='Plot a specific history file (optional)')
    
    args = parser.parse_args()
    
    if args.file:
        # Plot a specific file
        plot_history_file(args.file, args.output_dir, args.show)
    else:
        # Plot all files in the data directory
        plot_all_histories(args.data_dir, args.output_dir, args.show)

if __name__ == "__main__":
    main()