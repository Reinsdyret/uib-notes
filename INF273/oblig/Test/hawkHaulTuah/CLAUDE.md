# CLAUDE.md - Project Guide

## Build & Run Commands
- Build: `cargo build`
- Run main: `cargo run --release`
- Run Local Search: Uncomment lines 32-34 in main.rs, then `cargo run --release`
- Run Simulated Annealing: Uncomment lines 29-31 in main.rs, then `cargo run --release`
- Run specific crate: `cd src/[crate_name] && cargo run --release`

## Project Structure
- Multi-crate workspace with modules for different algorithms
- Data files in `src/data/` with format `Call_X_Vehicle_Y.txt`
- Solution represented as `Vec<Vec<u32>>`

## Code Style Guidelines
- Use 4-space indentation
- Follow Rust naming conventions: snake_case for variables/functions
- Prefer early returns with detailed error messages
- Avoid unnecessary memory allocation
- Keep functions focused on a single responsibility
- Document complex functions with comments
- Implement thorough error handling

## Algorithm Parameters
- Local Search: `run_local_search_report(filename, parallel)`
- Simulated Annealing: `run_simmulated_annealing_report(filename, parallel, prob, t_final)`
