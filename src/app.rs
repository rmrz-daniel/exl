use std::{vec, collections::HashMap};



#[derive(Debug)]
pub struct App {
	pub should_quit: bool,
	pub grid: Vec<Vec<String>>,
	pub undo_stack: Vec<GridState>,
	pub selected_row: usize,
	pub selected_col: usize,
	pub selected_cells: Option<HashMap< (usize,usize), String >>,
	pub current_mode: AppMode,
	pub input: String,
	pub cursor_pos: usize,
}

#[derive(Debug, Clone)]
pub struct GridState {
	pub grid: Vec<Vec<String>>
}

pub const DEFAULT_ROWS: usize = 30; 
pub const DEFAULT_COLS: usize = 10;
const MAX_UNDO_LEVELS: usize = 7;


impl Default for App {
	fn default() -> Self {
		App { 
			should_quit: false,
			grid: 
				vec![
					vec!["".to_string(); DEFAULT_COLS];
					DEFAULT_ROWS
				],
			undo_stack: vec![],
			selected_row: 0,
			selected_col: 0,
			selected_cells: None,
			input: "".to_string(),
			cursor_pos: 0,
			current_mode: AppMode::Navigation
		}
	}
}

#[derive(Debug)]
pub enum AppMode{
    Navigation,
    Editing,
    Selecting
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

	 pub fn tick(&self) {}

	 pub fn quit(&mut self) {
	 	self.should_quit = true;
	 }

 	pub fn select_cell(&mut self, row: usize, col: usize) {
        self.selected_row = row;
        self.selected_col = col;
    }

    pub fn edit_cell(&mut self, content: &str) {

    	let cloned_grid = GridState {
        	grid: self.grid.clone(),
    	};

        self.grid[self.selected_row][self.selected_col] = content.to_string();

    	if self.undo_stack.len() >= MAX_UNDO_LEVELS {
	        self.undo_stack.remove(0);
	    }
	    
	    self.undo_stack.push(cloned_grid);
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
		self.selected_cells = None;
	}
 
 	// Edit functions v

 	pub fn edit(&mut self) {
 		self.input = self.grid[self.selected_row][self.selected_col].clone();
 		self.current_mode = AppMode::Editing;
 		self.cursor_pos = self.input.len();
 	}


	pub fn cursor_left(&mut self) {
		self.cursor_pos = self.clamp(self.cursor_pos.saturating_sub(1))
	}

	pub fn cursor_right(&mut self) {
		self.cursor_pos = self.clamp(self.cursor_pos.saturating_add(1))
	}

	fn clamp(&self, new_pos: usize) -> usize {
		new_pos.clamp(0, self.input.len())
	}

	pub fn enter_char(&mut self, new_char: char) {
		self.input.insert(self.cursor_pos, new_char);
		self.cursor_right();
	}

	pub fn del_char(&mut self) {
		if self.cursor_pos != 0 {
			let left_of_del = self.input.chars().take(self.cursor_pos - 1);
			let right_of_del = self.input.chars().skip(self.cursor_pos);

			self.input = left_of_del.chain(right_of_del).collect();
			self.cursor_left()
		}
	}

	pub fn submit_changes(&mut self) {
		self.grid[self.selected_row][self.selected_col] = self.input.to_string();
		self.cursor_pos = 0;
		self.quit_mode()
	}

	// Edit functions ^

	// Select Functions v

	pub fn select(&mut self) {
		self.current_mode = AppMode::Selecting;
		let mut cell_map: HashMap< (usize,usize) , String> = HashMap::new();
		
		
		cell_map.insert( (self.selected_row, self.selected_col ), self.grid[self.selected_row][self.selected_col].clone() );
		

		self.selected_cells = Some(cell_map);
	}

	pub fn select_nav(&mut self, direction: ArrowKeys) {
		self.nav(direction);

		self.selected_cells.as_mut().unwrap().insert((self.selected_row, self.selected_col ), self.grid[self.selected_row][self.selected_col].clone());

		let (min_x, max_x, min_y, max_y) = self.selected_cells.as_ref().unwrap().iter().fold(None, |acc, ((x, y), _)| {
		    match acc {
		        None => Some((*x, *x, *y, *y)), //initialize to the first pair
		        Some((min_x, max_x, min_y, max_y)) => Some((min_x.min(*x), max_x.max(*x), min_y.min(*y), max_y.max(*y))),
		    }
		}).unwrap();

		for (row, row_vec) in self.grid.iter().enumerate() {
			for (col, content) in row_vec.iter().enumerate() {
				if !self.selected_cells.as_ref().unwrap().contains_key(&(row,col)) && row >= min_x && row <= max_x && col >= min_y && col <= max_y {
					self.selected_cells.as_mut().unwrap().insert((row,col), content.clone());
				}
			}
		}
				
	}

	// Select Functions ^

    // pub fn add_row(&mut self, row_index: usize) {
    //     // Implementation for adding a row
    // }

    // pub fn add_column(&mut self, col_index: usize) {
    //     // Implementation for adding a column
    // }

    // pub fn remove_row(&mut self, row_index: usize) {
    //     // Implementation for removing a row
    // }

    // pub fn remove_column(&mut self, col_index: usize) {
    //     // Implementation for removing a column
    // }

    pub fn undo(&mut self) {
    	if let Some(previous_state) = self.undo_stack.pop() {
    		self.grid = previous_state.grid;
    	}
    }


}