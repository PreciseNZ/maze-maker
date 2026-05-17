# Rust Maze Generator

A command-line maze generator built in Rust using a randomized depth-first search (backtracking) algorithm. The program generates a perfect square maze of any specified size and outputs it directly to the terminal using ASCII text formatting.

## Features

- **Dynamic Maze Generation:** Generates an $N \times N$ grid where all paths are reachable and there are no loops.
- **Customizable Size:** Pass the size of the maze as a command-line argument.
- **Solution Paths:** Optionally visualize the path taken by the generator using navigational directional markers (`^`, `v`, `<`, `>`).
- **Memory Efficient:** Leverages Rust's contiguous `Vec<T>` layout and stack-allocated primitives for optimal execution performance.

## Prerequisites

To compile and run this project, you must have the Rust toolchain installed. If you don't have it, install it via [rustup](https://rustup.rs/).

## Installation & Setup

1. Initialize your project directory if you haven't already:
   ```bash
   cargo init