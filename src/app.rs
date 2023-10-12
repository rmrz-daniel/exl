use std::{vec};

#[derive(Debug)]
pub struct App {
    pub should_quit: bool,
    pub grid: Vec<Vec<Cell>>,
    pub undo_stack: Vec<GridState>,
    pub selected_row: usize,
    pub selected_col: usize,
    pub selected_range: Option<MinMaxRange>,
    pub current_mode: AppMode,
    pub input: String,
    pub cursor_pos: usize,
}

#[derive(Debug, Clone)]
pub struct GridState {
    pub grid: Vec<Vec<Cell>>,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Cell {
	pub content: String,
	pub selected: bool
}

#[derive(Debug)]
pub struct MinMaxRange {
    pub min_x: usize,
    pub max_x: usize,
    pub min_y: usize,
    pub max_y: usize,
}

pub const DEFAULT_ROWS: usize = 30;
pub const DEFAULT_COLS: usize = 10;

impl Default for App {
    fn default() -> Self {
        App {
            should_quit: false,
            grid: vec![vec![Cell::default(); DEFAULT_COLS]; DEFAULT_ROWS],
            undo_stack: vec![],
            selected_row: 0,
            selected_col: 0,
            selected_range: None,
            input: "".to_string(),
            cursor_pos: 0,
            current_mode: AppMode::Navigation,
        }
    }
}

impl Default for Cell {
	fn default() -> Self {
		Cell { content: "".to_string(), selected: false }
    }
}

impl Cell {
	fn reset_selected(cells: &mut Vec<Vec<Cell>>) {
		for row in cells.iter_mut() {
			for cell in row.iter_mut() {
				cell.selected = false;
			}
		}
	}
}

#[derive(Debug)]
pub enum AppMode {
    Navigation,
    Editing,
    Selecting,
    SingleSelect,
    FormulaInput,
    Formula,
}

#[derive(Debug)]
pub enum ArrowKeys {
    Left,
    Right,
    Up,
    Down,
}


impl App {
    pub fn new() -> Self {
        Self::default()
    }

    // pub fn rerender_grid(&mut self, width: u16, height: u16) {
    //     let cell_width = 12;
    //     let mut desired_width = (width / cell_width) as usize;
    //     let desired_height = height as usize;

    //     // Handle extra column for remainder
    //     let has_remainder = width % cell_width != 0;

    //     // Adjust the number of rows
    //     if self.grid.len() < desired_height {
    //         // Add new rows
    //         let additional_rows = desired_height - self.grid.len();
    //         for _ in 0..additional_rows {
    //             self.grid.push(vec!["".to_string(); desired_width]);
    //         }
    //     } else if self.grid.len() > desired_height {
    //         // Remove extra rows
    //         self.grid.truncate(desired_height);
    //     }

    //     // Adjust the number of columns for each row
    //     for row in self.grid.iter_mut() {
    //         if has_remainder {
    //             // Add an extra column for the remainder
    //             row.push("".to_string());
    //             desired_width += 1;
    //         }

    //         if row.len() < desired_width {
    //             // Add new cells to the row
    //             let additional_cells = desired_width - row.len();
    //             for _ in 0..additional_cells {
    //                 row.push("".to_string());
    //             }
    //         } else if row.len() > desired_width {
    //             // Remove extra cells from the row
    //             row.truncate(desired_width);
    //         }
    //     }
    // }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn nav(&mut self, direction: ArrowKeys) {
        let col_amount = self.grid.len();
        let row_amount = self.grid[self.selected_row].len();

        // If a % b = 0, it means a is evenly divisible by b. IE the start
        // If a % b is less than b, it means a is not evenly divisible by b, and the remainder is less than b. IE always within range of [0, b-1]
        match direction {
            ArrowKeys::Left => self.selected_col = (self.selected_col + row_amount - 1) % row_amount,
            ArrowKeys::Right => self.selected_col = (self.selected_col + 1) % row_amount,
            ArrowKeys::Up => self.selected_row = (self.selected_row + col_amount - 1) % col_amount,
            ArrowKeys::Down => self.selected_row = (self.selected_row + 1) % col_amount,
        }
    }

    pub fn quit_mode(&mut self) {
        self.current_mode = AppMode::Navigation;
        self.input.clear();
        self.selected_range = None;
        Cell::reset_selected(&mut self.grid);
    }

    // Formula Functions v

    pub fn formula(&mut self) {
        self.current_mode = AppMode::Formula;
    }

    pub fn formula_input(&mut self) {
        self.current_mode = AppMode::FormulaInput;
    }

    // Formula Functions ^

    pub fn undo(&mut self) {
        if let Some(previous_state) = self.undo_stack.pop() {
            self.grid = previous_state.grid;
        }
    }
}
