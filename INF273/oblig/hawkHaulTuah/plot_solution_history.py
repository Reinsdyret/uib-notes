#!/usr/bin/env python3
import os
import glob
import pandas as pd
import matplotlib.pyplot as plt
import argparse
from matplotlib.ticker import ScalarFormatter
import numpy as np

def plot_history_file(file_path, output_dir="plots", show_plots=False, log_scale=False):
    """
    Plot the best and incumbent costs from a history CSV file and mark the last improvement.

    Args:
        file_path: Path to the CSV file
        output_dir: Directory to save plot images
        show_plots: Whether to display plots interactively
        log_scale: Use logarithmic scale for y-axis
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

    # Find the iteration where the best cost stopped improving
    best_costs = data['best_cost'].values
    last_improvement_idx = 0
    last_best_cost = best_costs[0]

    for i in range(1, len(best_costs)):
        if best_costs[i] < last_best_cost:
            last_improvement_idx = i
            last_best_cost = best_costs[i]

    last_improvement_iteration = data['iteration'].iloc[last_improvement_idx]
    last_improvement_cost = best_costs[last_improvement_idx]

    # Mark the last improvement with a point and text annotation
    plt.scatter(last_improvement_iteration, last_improvement_cost,
                color='green', s=100, zorder=5, label='Last Improvement')

    # Add annotation with arrow
    plt.annotate(f'Last Improvement\nIteration: {last_improvement_iteration}\nCost: {last_improvement_cost:.2e}',
                xy=(last_improvement_iteration, last_improvement_cost),
                xytext=(last_improvement_iteration + len(data) * 0.05,
                        last_improvement_cost * (1.1 if not log_scale else 1.3)),
                arrowprops=dict(facecolor='black', shrink=0.05, width=1.5, headwidth=8),
                bbox=dict(boxstyle="round,pad=0.5", fc="yellow", alpha=0.7))

    # Add labels and title
    plt.xlabel('Iteration')
    plt.ylabel('Cost')
    plt.title(f'Solution Progress for {problem_name}')

    # Set y-axis to log scale if requested
    if log_scale:
        plt.yscale('log')
        plt.ylabel('Cost (log scale)')
    else:
        # Use scientific notation for large numbers
        plt.gca().yaxis.set_major_formatter(ScalarFormatter(useMathText=True))
        plt.ticklabel_format(style='sci', axis='y', scilimits=(0,0))

    plt.legend()

    # Add grid
    plt.grid(True, linestyle='--', alpha=0.7)

    # Save the plot
    output_path = os.path.join(output_dir, f"{problem_name}_plot{'_log' if log_scale else ''}.png")
    plt.savefig(output_path, dpi=150, bbox_inches='tight')
    print(f"Plot saved to {output_path}")

    if show_plots:
        plt.show()
    else:
        plt.close()

def plot_all_histories(data_dir="output", output_dir="plots", show_plots=False, log_scale=False):
    """
    Plot all history CSV files in the given directory.

    Args:
        data_dir: Directory containing the history CSV files
        output_dir: Directory to save plot images
        show_plots: Whether to display plots interactively
        log_scale: Use logarithmic scale for y-axis
    """
    # Find all history CSV files
    history_files = glob.glob(os.path.join(data_dir, "*_history.csv"))

    if not history_files:
        print(f"No history CSV files found in {data_dir}")
        return

    print(f"Found {len(history_files)} history files")

    # Plot each file
    for file_path in history_files:
        plot_history_file(file_path, output_dir, show_plots, log_scale)

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
    parser.add_argument('--log', action='store_true',
                        help='Use logarithmic scale for y-axis')

    args = parser.parse_args()

    if args.file:
        # Plot a specific file
        plot_history_file(args.file, args.output_dir, args.show, args.log)
    else:
        # Plot all files in the data directory
        plot_all_histories(args.data_dir, args.output_dir, args.show, args.log)

if __name__ == "__main__":
    main()