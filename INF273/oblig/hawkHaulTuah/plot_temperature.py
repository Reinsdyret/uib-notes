#!/usr/bin/env python3
import os
import glob
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
import argparse
from matplotlib.ticker import ScalarFormatter

def plot_temperature(temperature_file, output_dir="plots", show_plots=False, log_scale=True):
    """
    Plot the temperature history from a CSV file.
    
    Args:
        temperature_file: Path to the CSV file containing temperature data
        output_dir: Directory to save plot images
        show_plots: Whether to display plots interactively
        log_scale: Use logarithmic scale for y-axis (typically better for temperature)
    """
    # Create output directory if it doesn't exist
    os.makedirs(output_dir, exist_ok=True)
    
    # Get the problem name from the file path
    file_name = os.path.basename(temperature_file)
    problem_name = file_name.replace("_temperature.csv", "")
    
    # Read the temperature CSV file
    temp_data = pd.read_csv(temperature_file)
    
    # Setup the plot with a white background
    plt.figure(figsize=(12, 8), facecolor='white')
    ax = plt.gca()
    ax.set_facecolor('white')
    
    # Plot temperature
    plt.plot(temp_data['iteration'], temp_data['temperature'], 
             color='red', linewidth=2, label='Temperature')
    
    # Add labels and title with better styling
    plt.xlabel('Iteration', fontsize=12, fontweight='bold')
    plt.ylabel('Temperature' + (' (log scale)' if log_scale else ''), 
              fontsize=12, fontweight='bold')
    plt.title(f'Temperature Cooling Schedule for {problem_name}', 
              fontsize=14, fontweight='bold')
    
    # Set y-axis to log scale if requested (recommended for temperature)
    if log_scale:
        plt.yscale('log')
    
    # Add grid with lighter styling
    plt.grid(True, linestyle='--', alpha=0.3, color='gray')
    
    # Improve tick labels
    plt.xticks(fontsize=10)
    plt.yticks(fontsize=10)
    
    # Add a box around the plot
    plt.box(True)
    
    # Add legend
    plt.legend(loc='upper right', fontsize=10)
    
    # Save the plot with higher resolution
    output_path = os.path.join(output_dir, f"{problem_name}_temperature_plot{'_log' if log_scale else ''}.png")
    plt.savefig(output_path, dpi=200, bbox_inches='tight', facecolor='white')
    print(f"Plot saved to {output_path}")
    
    if show_plots:
        plt.show()
    else:
        plt.close()

def plot_all_temperatures(data_dir="output", output_dir="plots", show_plots=False, log_scale=True):
    """
    Plot all temperature CSV files in the given directory.
    
    Args:
        data_dir: Directory containing the temperature CSV files
        output_dir: Directory to save plot images
        show_plots: Whether to display plots interactively
        log_scale: Use logarithmic scale for y-axis
    """
    # Find all temperature CSV files
    temperature_files = glob.glob(os.path.join(data_dir, "*_temperature.csv"))
    
    if not temperature_files:
        print(f"No temperature CSV files found in {data_dir}")
        return
    
    print(f"Found {len(temperature_files)} temperature files")
    
    # Plot each file
    for temperature_file in temperature_files:
        plot_temperature(temperature_file, output_dir, show_plots, log_scale)

def main():
    parser = argparse.ArgumentParser(description='Plot temperature history from CSV files')
    parser.add_argument('--data-dir', default='output', 
                        help='Directory containing temperature CSV files (default: output)')
    parser.add_argument('--output-dir', default='plots', 
                        help='Directory to save plot images (default: plots)')
    parser.add_argument('--show', action='store_true', 
                        help='Show plots interactively')
    parser.add_argument('--file', 
                        help='Plot a specific temperature file (optional)')
    parser.add_argument('--no-log', action='store_true',
                        help='Disable logarithmic scale for y-axis')
    
    args = parser.parse_args()
    log_scale = not args.no_log
    
    if args.file:
        # Plot a specific file
        plot_temperature(args.file, args.output_dir, args.show, log_scale)
    else:
        # Plot all files in the data directory
        plot_all_temperatures(args.data_dir, args.output_dir, args.show, log_scale)

if __name__ == "__main__":
    main()