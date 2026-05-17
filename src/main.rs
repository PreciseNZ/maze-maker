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
            wall_bottom: true,
            wall_top: true,
            wall_left: true,
            wall_right: true,
            move_direction: '?',

        }
    }
}

struct Maze {
    cells: Vec<Cell>,
    solution: Vec<Cell>,
    backstack: Vec<usize>,
    size: usize,
    current_cell: usize,
    is_completed: bool,
    show_solution: bool,
}

impl Maze {
    fn new() -> Self {
        Self {
            cells: vec![],
            solution: vec![],
            backstack: vec![],
            size: 0,
            current_cell: 0,
            is_completed: false,
            show_solution: false,
        }
    }

    pub fn create_maze(&mut self, size: usize) {
        self.size = size;
        self.cells = Vec::new();
        self.backstack = Vec::new();
        self.solution = Vec::new();

        print!("Creating maze... ");
        for r in 0..self.size {
            for c in 0..self.size {
                // cells.len() provides the current count as a usize
                let id = self.cells.len();
                self.cells.push(Cell::new(id, c, r));
            }
        }
        print!("with {} cells...", self.cells.len());

        self.add_boundary_walls();

        self.current_cell = 0;
        self.cells[0].is_visited = true;
        self.cells[0].is_start = true;

        let last_index = self.cells.len() - 1;
        self.cells[last_index].is_end = true;
        while !self.is_completed {
            self.update_maze();
        }
        println!("done! Maze creation is complete.");
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
        let neighbours = self.get_available_neighbours(current_row, current_col);

        if neighbours.is_empty() {
            if !self.backstack.is_empty() {
                self.cells[self.current_cell].is_visited = true;
                self.cells[self.current_cell].is_current = false;
                if let Some(back_stack_id) = self.backstack.pop() {
                    self.cells[back_stack_id].is_current = true;
                    self.current_cell = back_stack_id;
                }
            }
            else {
                self.is_completed = true;
            }
        } else {
            let random_number = rand::random_range(0..neighbours.len());
            let next_cell = neighbours[random_number];

            for neighbour in neighbours {
                if let Some(target_left_cell) = current_cell.checked_sub(1) {
                    if neighbour == target_left_cell {
                        if neighbour == next_cell {
                            self.cells[self.current_cell].move_direction = '<';
                            self.cells[current_cell].wall_left = false;
                            self.cells[neighbour].wall_right = false;
                        }
                    }
                }

                if let Some(target_right) = self.get_linear_index(current_row, current_col + 1) {
                    if neighbour == target_right {
                        if neighbour == next_cell {
                            self.cells[self.current_cell].move_direction = '>';
                            self.cells[current_cell].wall_right = false;
                            self.cells[neighbour].wall_left = false;
                        }
                    }
                }

                if let Some(target_top_cell) = current_cell.checked_sub(self.size) {
                    if neighbour == target_top_cell {
                        if neighbour == next_cell {
                            self.cells[self.current_cell].move_direction = '^';
                            self.cells[current_cell].wall_top = false;
                            self.cells[neighbour].wall_bottom = false;
                        }
                    }
                }

                if let Some(target_bottom) = self.get_linear_index(current_row + 1, current_col) {
                    if neighbour == target_bottom {
                        if neighbour == next_cell {
                            self.cells[self.current_cell].move_direction = 'v';
                            self.cells[current_cell].wall_bottom = false;
                            self.cells[neighbour].wall_top = false;
                        }
                    }
                }
            }
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
        // Top and Bottom boundary loops
        for col in 0..self.size {
            if let Some(top_idx) = self.get_linear_index(0, col) {
                self.cells[top_idx].wall_top = true;
            }
            if let Some(bottom_idx) = self.get_linear_index(self.size - 1, col) {
                self.cells[bottom_idx].wall_bottom = true;
            }
        }

        // Left and Right boundary loops
        for row in 0..self.size {
            if let Some(left_idx) = self.get_linear_index(row, 0) {
                self.cells[left_idx].wall_left = true;
            }
            if let Some(right_idx) = self.get_linear_index(row, self.size - 1) {
                self.cells[right_idx].wall_right = true;
            }
        }
    }

    pub fn get_linear_index(&self, row_index: usize, column_index: usize) -> Option<usize> {
        if row_index >= self.size || column_index >= self.size {
            return None; // Index is out-of-bounds
        }

        Some((row_index * self.size) + column_index)
    }

    pub fn get_available_neighbours(&self, cell_row: usize, cell_column: usize) -> Vec<usize> {
        let mut neighbours = Vec::new();

        // Force last cell to have no neighbours
        if let Some(target_end_cell) = self.get_linear_index(cell_row, cell_column) {
            if target_end_cell == self.cells.len() - 1 {
                return neighbours;
            }
        }

        if let Some(target_column) = cell_column.checked_sub(1) {
            if let Some(left_idx) = self.get_linear_index(cell_row, target_column) {
                if !self.cells[left_idx].is_visited {
                    neighbours.push(left_idx);
                }

            }
        }
        if let Some(right_idx) = self.get_linear_index(cell_row, cell_column + 1) {
            if !self.cells[right_idx].is_visited {
                neighbours.push(right_idx);
            }
        }

        if let Some(target_row) = cell_row.checked_sub(1) {
            if let Some(top_idx) = self.get_linear_index(target_row, cell_column) {
                if !self.cells[top_idx].is_visited {
                    neighbours.push(top_idx);
                }
            }
        }

        if let Some(bottom_idx) = self.get_linear_index(cell_row + 1, cell_column) {
            if !self.cells[bottom_idx].is_visited {
                neighbours.push(bottom_idx);
            }
        }

        neighbours
    }

    pub fn print_maze(&mut self) {
        for row in 0..self.size {
            // Print the top of the cell.
            for col in 0..self.size {
                if let Some(cell) = self.get_linear_index(row, col) {
                    print!("{}", self.print_top(cell));
                }
            }
            print!("+");
            println!();
            // Print the middle of the cell.
            for col in 0..self.size {
                if let Some(cell) = self.get_linear_index(row, col) {
                    print!("{}", self.print_mid(cell));
                }
            }
            if row == self.size - 1 {
                print!(">");
            } else {
                print!("|");
            }
            println!();
        }
        // Print the bottom of the maze (bottom will fit the top.
        for _col in 0..self.size {
            print!("{}", self.print_bottom());
        }
        print!("+");
        println!();
    }

    pub fn print_top(&mut self, cell_index: usize) -> &'static str {
        let cell = &self.cells[cell_index];
        if cell.wall_top {
            "+---"
        } else {
            "+   "
        }
    }
    pub fn print_bottom(&mut self) -> &'static str {
        "+---"
    }

    pub fn print_mid(&mut self, cell_index: usize) -> String {
        let cell = &self.cells[cell_index];
        let mut result = String::with_capacity(3);
        if cell.wall_left {
            if cell.is_start {
                result.push_str(">");
            } else {
                result.push_str("|");
            }
        } else {
            result.push_str(" ");
        }

        if self.show_solution && cell.is_solution {
            result.push_str(" ");
            result.push(cell.move_direction);
            result.push_str(" ");
        } else {
            result.push_str("   ");
        }

        result
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut maze = Maze::new();
    let mut maze_size: usize = 5;
    if let Some(val_str) = args.get(1) {
        match val_str.parse::<usize>() {
            Ok(num) => maze_size = num,
            Err(_) => eprintln!("Usage: cargo run <number> <--show-solution>"),
        }
    } else {
        eprintln!("Defaulting to 10x10.");
        eprintln!("Usage: cargo run <number> <--show-solution>");
    }
    if let Some(val_str) = args.get(2) {
        if val_str == "--show-solution" {
            maze.show_solution = true;
        }
    }

    maze.create_maze(maze_size);

    maze.print_maze();
}
