#!/usr/bin/env python3
import os
import glob
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
import argparse
from matplotlib.ticker import ScalarFormatter

def plot_probability(probability_file, temperature_file=None, output_dir="plots", show_plots=False):
    """
    Plot the probability history from a CSV file.
    
    Args:
        probability_file: Path to the CSV file containing probability data
        temperature_file: Optional path to temperature file to plot on secondary axis
        output_dir: Directory to save plot images
        show_plots: Whether to display plots interactively
    """
    # Create output directory if it doesn't exist
    os.makedirs(output_dir, exist_ok=True)
    
    # Get the problem name from the file path
    file_name = os.path.basename(probability_file)
    problem_name = file_name.replace("_probability.csv", "")
    
    # Read the probability CSV file
    prob_data = pd.read_csv(probability_file)
    
    # Setup the plot with a white background
    plt.figure(figsize=(12, 8), facecolor='white')
    ax1 = plt.gca()
    ax1.set_facecolor('white')
    
    # Plot probability
    ax1.scatter(prob_data['iteration'], prob_data['probability'], 
               color='blue', alpha=0.7, s=30, label='Acceptance Probability')
    
    # If we have temperature data, plot it on a secondary y-axis
    if temperature_file and os.path.exists(temperature_file):
        temp_data = pd.read_csv(temperature_file)
        ax2 = ax1.twinx()
        ax2.plot(temp_data['iteration'], temp_data['temperature'], 
                 color='red', linewidth=1.5, alpha=0.6, label='Temperature')
        ax2.set_ylabel('Temperature', fontsize=12, color='red')
        ax2.tick_params(axis='y', labelcolor='red')
        # Set y-axis to log scale for temperature
        # ax2.set_yscale('log')
        
        # Add separate legend for temperature
        lines2, labels2 = ax2.get_legend_handles_labels()
        ax2.legend(lines2, labels2, loc='upper right', fontsize=10)
    
    # Add labels and title with better styling
    ax1.set_xlabel('Iteration', fontsize=12, fontweight='bold')
    ax1.set_ylabel('Probability', fontsize=12, fontweight='bold', color='blue')
    ax1.tick_params(axis='y', labelcolor='blue')
    plt.title(f'Acceptance Probability for Non-Improving Moves in {problem_name}', 
              fontsize=14, fontweight='bold')
    
    # Add grid with lighter styling
    ax1.grid(True, linestyle='--', alpha=0.3, color='gray')
    
    # Improve tick labels
    plt.xticks(fontsize=10)
    ax1.tick_params(axis='y', labelsize=10)
    
    # Add a box around the plot
    plt.box(True)
    
    # Add legend for probability
    lines1, labels1 = ax1.get_legend_handles_labels()
    ax1.legend(lines1, labels1, loc='upper left', fontsize=10)
    
    # Save the plot with higher resolution
    output_path = os.path.join(output_dir, f"{problem_name}_probability_plot.png")
    plt.savefig(output_path, dpi=200, bbox_inches='tight', facecolor='white')
    print(f"Plot saved to {output_path}")
    
    if show_plots:
        plt.show()
    else:
        plt.close()

def plot_all_probabilities(data_dir="output", output_dir="plots", show_plots=False):
    """
    Plot all probability CSV files in the given directory.
    
    Args:
        data_dir: Directory containing the probability CSV files
        output_dir: Directory to save plot images
        show_plots: Whether to display plots interactively
    """
    # Find all probability CSV files
    probability_files = glob.glob(os.path.join(data_dir, "*_probability.csv"))
    
    if not probability_files:
        print(f"No probability CSV files found in {data_dir}")
        return
    
    print(f"Found {len(probability_files)} probability files")
    
    # Plot each file
    for probability_file in probability_files:
        # Find corresponding temperature file if it exists
        base_name = probability_file.replace("_probability.csv", "")
        temperature_file = f"{base_name}_temperature.csv"
        
        if os.path.exists(temperature_file):
            plot_probability(probability_file, temperature_file, output_dir, show_plots)
        else:
            plot_probability(probability_file, None, output_dir, show_plots)

def main():
    parser = argparse.ArgumentParser(description='Plot acceptance probability from CSV files')
    parser.add_argument('--data-dir', default='output', 
                        help='Directory containing probability CSV files (default: output)')
    parser.add_argument('--output-dir', default='plots', 
                        help='Directory to save plot images (default: plots)')
    parser.add_argument('--show', action='store_true', 
                        help='Show plots interactively')
    parser.add_argument('--file', 
                        help='Plot a specific probability file (optional)')
    parser.add_argument('--temp-file',
                        help='Optional temperature file to plot on secondary axis (use with --file)')
    
    args = parser.parse_args()
    
    if args.file:
        # Plot a specific file
        plot_probability(args.file, args.temp_file, args.output_dir, args.show)
    else:
        # Plot all files in the data directory
        plot_all_probabilities(args.data_dir, args.output_dir, args.show)

if __name__ == "__main__":
    main()