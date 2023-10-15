
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
    pub view_bound: (usize, usize),
    pub cell_amount: (usize, usize)
}

#[derive(Debug, Clone)]
pub struct GridState {
    pub grid: Vec<Vec<Cell>>,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Cell {
	pub content: String,
	pub selected: bool,
	pub header: bool,
}

#[derive(Debug)]
pub struct MinMaxRange {
    pub min_x: usize,
    pub max_x: usize,
    pub min_y: usize,
    pub max_y: usize,
}

pub const CELL_WIDTH: u16 = 12;

impl Default for App {
    fn default() -> Self {
        App {
            should_quit: false,
            grid: vec![vec![Cell::default(); 27]; 200],
            undo_stack: vec![],
            selected_row: 1,
            selected_col: 1,
            selected_range: None,
            input: "".to_string(),
            cursor_pos: 0,
            current_mode: AppMode::Navigation,
            view_bound: (0,0),
            cell_amount: (0,0),
        }
    }
}

impl Default for Cell {
	fn default() -> Self {
		Cell { content: "".to_string(), selected: false, header: false}
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
    	App::default()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn index_to_excel_column(index: usize) -> String {
	    if index == 0 {
	        return String::from("A");
	    }

	    let mut result = String::new();
	    let mut n = index + 1;

	    while n > 0 {
	        n -= 1;
	        let remainder = n % 26;
	        let letter = (remainder as u8 + b'A') as char;
	        result.push(letter);
	        n /= 26;
	    }

	    result.chars().rev().collect()
	}

	pub fn header(&mut self) {
		// let x = self.grid[0].clone().iter_mut().enumerate().map(| (index, cell)| cell.content = Self::index_to_excel_column(index)).collect();

		for (index, cell) in self.grid[0].iter_mut().skip(1).enumerate() {
			cell.content = Self::index_to_excel_column(index);
			cell.header = true;
		}

		for (index, row) in self.grid.iter_mut().enumerate() {
			if index != 0 { row[0].content =  index.to_string() };
			row[0].header = true;
		}

	}

    pub fn insert_row(&mut self, index: usize) {
    	let col_count = self.grid.first().unwrap().len();
    	self.grid.insert(index, vec![Cell::default(); col_count] );
    	self.header()
    }

    pub fn insert_col(&mut self, index: usize) {
    	self.grid.iter_mut().for_each(|row| row.insert(index, Cell::default()));
    	self.header()
    }

    pub fn nav(&mut self, direction: ArrowKeys) {
        let row_amount = self.grid.len();
        let col_amount = self.grid[self.selected_row].len();

        match direction {
            ArrowKeys::Left => {
            	if self.selected_col > 1 {
            		self.selected_col -= 1;
            	} else {
            		self.view_bound.0 = 0;
            	}
            },
            ArrowKeys::Right => {
            	if self.selected_col < col_amount - 1 {
            		self.selected_col += 1;
            	}
            },
            ArrowKeys::Up => {
            	if self.selected_row > 1 {
            		self.selected_row -= 1;
            	} else {
            		self.view_bound.1 = 0;

            	}
            },
            ArrowKeys::Down => {
            	if self.selected_row < row_amount - 1 {
            		self.selected_row += 1;
            	}
            },
        }

        if self.selected_col < self.view_bound.0 {
            self.view_bound.0 = self.selected_col;
        } else if self.selected_col >= self.view_bound.0 + self.cell_amount.0 {
            self.view_bound.0 = self.selected_col - self.cell_amount.0 + 1;
        }

        if self.selected_row < self.view_bound.1 {
            self.view_bound.1 = self.selected_row;
        } else if self.selected_row >= self.view_bound.1 + self.cell_amount.1 {
            self.view_bound.1 = self.selected_row - self.cell_amount.1 + 1;
        }
	}
    pub fn quit_mode(&mut self) {
        self.current_mode = AppMode::Navigation;
        self.input.clear();
        self.selected_range = None;
        Cell::reset_selected(&mut self.grid);
    }

    pub fn undo(&mut self) {
        if let Some(previous_state) = self.undo_stack.pop() {
            self.grid = previous_state.grid;
        }
    }

    // pub fn init_table_grid(&mut self, width: u16, height: u16){
    // 	Hard coded the values of 2 and 4 for the margins for now
    // 	let w: usize = (width - 2).into();
    // 	let h: usize = (height - 4).into();

    // 	let cell_amount = w / CELL_WIDTH as usize;

    // 	if w % CELL_WIDTH as usize != 0{
    // 		// self.grid = vec![vec![Cell::default(); cell_amount + 1]; h];
    // 		self.cell_amount = (cell_amount + 1, h);
    // 	} else {
    // 		// self.grid = vec![vec![Cell::default(); cell_amount]; h];
    // 		self.cell_amount = (cell_amount, h);
    // 	}

    // }

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
}
