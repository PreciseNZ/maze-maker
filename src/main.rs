use std::env;

struct Cell {
    id: usize,
    col: usize,
    row: usize,

    is_current: bool,
    is_visited: bool,
    is_start: bool,
    is_end: bool,
    is_solution: bool,

    wall_bottom: bool,
    wall_top: bool,
    wall_left: bool,
    wall_right: bool,
    move_direction: char,
}

impl Cell {
    fn new(id: usize, col: usize, row: usize) -> Self {
        Self {
            id,
            col,
            row,
            is_current: false,
            is_visited: false,
            is_start: false,
            is_end: false,
            is_solution: false,
            wall_bottom: false,
            wall_top: false,
            wall_left: false,
            wall_right: false,
            move_direction: ' ',

        }
    }

}

struct Maze {
    cells: Vec<Cell>,
    solution: Vec<Cell>,
    backstack: Vec<usize>,
    columns: usize,
    rows: usize,
    current_cell: usize,
    is_completed: bool,
}

impl Maze {
    fn new() -> Self {
        Self {
            cells: vec![],
            solution: vec![],
            backstack: vec![],
            columns: 0,
            rows: 0,
            current_cell: 0,
            is_completed: false,
        }
    }

    pub fn create_maze(&mut self, number_of_rows: usize, number_of_columns: usize) {
        self.rows = number_of_rows;
        self.columns = number_of_columns;
        self.cells = Vec::new();
        self.backstack = Vec::new();
        self.solution = Vec::new();

        println!("Creating maze");
        for i in 0..self.columns {
            for j in 0..self.rows {
                // cells.len() provides the current count as a usize
                let id = self.cells.len();
                self.cells.push(Cell::new(id, i, j));
            }
        }
        println!("Maze created with {} cells.", self.cells.len());

        self.add_boundary_walls();

        self.current_cell = 0;
        self.cells[0].is_visited = true;
        self.cells[0].is_start = true;

        let last_index = self.cells.len() - 1;
        self.cells[last_index].is_end = true;

        while !self.is_completed {
            self.update_maze();
        }
        println!("Maze creation is complete.");
    }

    pub fn update_maze(&mut self) {
        if self.cells.is_empty() {
            println!("Maze is empty?  Is this exepected?");
            self.is_completed = true;
            return;
        }

        let current_cell = self.cells[self.current_cell].id;
        let current_row = self.cells[self.current_cell].row;
        let current_col = self.cells[self.current_cell].col;
        if current_cell == self.cells.len() -1 {
            self.is_completed = true;
            return;
        }

        let neighbors = self.get_available_neighbors(current_row, current_col);

        if neighbors.is_empty() {
            if !self.backstack.is_empty() {
                self.cells[self.current_cell].is_visited = true;
                self.cells[self.current_cell].is_current = false;
                if let Some(back_stack_id) = self.backstack.pop() {
                    self.cells[back_stack_id].is_current = true;
                    self.current_cell = back_stack_id;
                }
            }
            else {
                println!("No more neighbors, and backstack is empty.");
                self.is_completed = true;
            }
        } else {
            let random_number = rand::random_range(0..neighbors.len());
            print!("Cell: {}, Neighbors: {}, ", self.current_cell, neighbors.len());
            let next_cell = neighbors[random_number];

            for neighbor in neighbors {
                print!("Neighbor: {}, ", neighbor);
                if neighbor == next_cell {
                    print!("Picked: {}, ", neighbor);
                    // continue;
                }
                if let Some(target_left_cell) = current_cell.checked_sub(1) {
                    if neighbor == target_left_cell {
                        if neighbor == next_cell {
                            self.cells[self.current_cell].move_direction = '<';
                            continue;
                        }
                        self.cells[current_cell].wall_left = true;
                        self.cells[neighbor].wall_right = true;
                        print!("Left({}), ", neighbor);
                    }
                }
                if neighbor == current_cell + 1 {
                    if neighbor == next_cell {
                        self.cells[self.current_cell].move_direction = '>';
                        continue;
                    }
                    self.cells[current_cell].wall_right = true;
                    self.cells[neighbor].wall_left = true;
                    print!("Right({}), ", neighbor);
                }
                if let Some(target_top_cell) = current_cell.checked_sub(self.columns) {
                    if neighbor == target_top_cell {
                        if neighbor == next_cell {
                            self.cells[self.current_cell].move_direction = '^';
                            continue;
                        }
                        self.cells[current_cell].wall_top = true;
                        self.cells[neighbor].wall_bottom = true;
                        print!("Top({}), ", neighbor);
                    }
                }
                if neighbor == current_cell + self.columns {
                    if neighbor == next_cell {
                        self.cells[self.current_cell].move_direction = 'v';
                        continue;
                    }
                    self.cells[current_cell].wall_bottom = true;
                    self.cells[neighbor].wall_top = true;
                    print!("Bottom({}), ", neighbor);
                }
            }
            println!();
            self.cells[current_cell].is_visited = true;
            self.cells[current_cell].is_current = false;
            self.cells[self.current_cell].is_solution = true;
            self.backstack.push(self.current_cell);
            self.cells[next_cell].is_visited = true;
            self.cells[next_cell].is_current = true;
            self.current_cell = next_cell;
        }
    }

    pub fn add_boundary_walls(&mut self) {
        // 1. Top and Bottom boundary loops
        for col in 0..self.columns {
            if let Some(top_idx) = self.get_linear_index(0, col) {
                self.cells[top_idx].wall_top = true;
            }
            if let Some(bottom_idx) = self.get_linear_index(self.rows - 1, col) {
                self.cells[bottom_idx].wall_bottom = true;
            }
        }

        // 2. Left and Right boundary loops
        for row in 0..self.rows {
            if let Some(left_idx) = self.get_linear_index(row, 0) {
                self.cells[left_idx].wall_left = true;
            }
            if let Some(right_idx) = self.get_linear_index(row, self.columns - 1) {
                self.cells[right_idx].wall_right = true;
            }
        }
    }

    pub fn get_linear_index(&self, row_index: usize, column_index: usize) -> Option<usize> {
        if row_index >= self.rows || column_index >= self.columns {
            return None; // Index is out-of-bounds
        }

        Some(row_index * self.columns + column_index)
    }

    pub fn get_available_neighbors(&self, cell_row: usize, cell_column: usize) -> Vec<usize> {
        let mut neighbors = Vec::new();
        if let Some(target_column) = cell_column.checked_sub(1) {
            if let Some(top_idx) = self.get_linear_index(cell_row, target_column) {
                if !self.cells[top_idx].is_visited {
                    neighbors.push(top_idx);
                }
            }
        }
        if let Some(bottom_idx) = self.get_linear_index(cell_row, cell_column + 1) {
            if !self.cells[bottom_idx].is_visited {
                neighbors.push(bottom_idx);
            }
        }

        if let Some(target_row) = cell_row.checked_sub(1) {
            if let Some(left_idx) = self.get_linear_index(target_row, cell_column) {
                if !self.cells[left_idx].is_visited {
                    neighbors.push(left_idx);
                }
            }
        }

        if let Some(right_idx) = self.get_linear_index(cell_row + 1, cell_column) {
            if !self.cells[right_idx].is_visited {
                neighbors.push(right_idx);
            }
        }

        neighbors
    }

    pub fn print_maze(&mut self) {
        for row in 0..self.rows {
            // Print the top of the cell.
            for col in 0..self.columns {
                if let Some(cell) = self.get_linear_index(row, col) {
                    print!("{}", self.print_top(cell));
                }
            }
            print!("+");
            println!();
            // Print the middle of the cell.
            for col in 0..self.columns {
                if let Some(cell) = self.get_linear_index(row, col) {
                    print!("{}", self.print_mid(cell));
                }
            }
            if row == self.rows - 1 {
                print!("E");
            } else {
                print!("|");
            }
            println!();
        }
        // Print the bottom of the maze (bottom will fit the top.
        for _col in 0..self.columns {
            print!("{}", self.print_bottom());
        }
        print!("+");
        println!();
    }

    pub fn print_top(&mut self, cell_index: usize) -> &'static str {
        let cell = &self.cells[cell_index];
        if cell.wall_top {
            "+-"
        } else {
            "+ "
        }
    }
    pub fn print_bottom(&mut self) -> &'static str {
        "+-"
    }

    pub fn print_mid(&mut self, cell_index: usize) -> String {
        let cell = &self.cells[cell_index];
        let mut result = String::with_capacity(3);
        if cell.wall_left {
            if cell.is_start {
                result.push_str("S");
            } else {
                result.push_str("|");
            }
        } else {
            result.push_str(" ");
        }

        // if cell.is_current {
        //     result.push_str("X");
        // } else if cell.is_start {
        //     result.push_str("S");
        // } else if cell.is_end {
        //     result.push_str("E");
        // } else {
        //     result.push_str(cell.id.to_string().as_str());
        // }

        if cell.is_visited {
            result.push(cell.move_direction);
        } else {
            result.push_str("0");
        }
        // result.push_str(" ");
        result
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut maze = Maze::new();
    let mut maze_size: usize = 10;
    if let Some(val_str) = args.get(1) {
        // Parse the String into a usize
        match val_str.parse::<usize>() {
            Ok(num) => maze_size = num,
            Err(_) => eprintln!("Error: Please provide a valid positive integer."),
        }
    } else {
        eprintln!("Usage: cargo run <number>");
    }

    maze.create_maze(maze_size, maze_size);

    maze.print_maze();
}
