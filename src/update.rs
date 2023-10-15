use std::todo;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::app::AppMode;
use crate::modes::edit::*;
use crate::modes::select::*;
use crate::app::App;

fn arrow_helper(app: &mut App, key_event: KeyEvent) {
    if key_event.modifiers == KeyModifiers::SHIFT {
        select(app)
    } else {
        match app.current_mode {
            crate::app::AppMode::Navigation | crate::app::AppMode::Formula | crate::app::AppMode::SingleSelect => match key_event.code {
                KeyCode::Right => app.nav(crate::app::ArrowKeys::Right),
                KeyCode::Left => app.nav(crate::app::ArrowKeys::Left),
                KeyCode::Down => app.nav(crate::app::ArrowKeys::Down),
                KeyCode::Up => app.nav(crate::app::ArrowKeys::Up),
                _ => {}
            },
            crate::app::AppMode::Selecting => match key_event.code {
                KeyCode::Right => select_nav(app, crate::app::ArrowKeys::Right),
                KeyCode::Left => select_nav(app, crate::app::ArrowKeys::Left),
                KeyCode::Down => select_nav(app, crate::app::ArrowKeys::Down),
                KeyCode::Up => select_nav(app, crate::app::ArrowKeys::Up),
                _ => {}
            },
            crate::app::AppMode::Editing => {
                match key_event.code {
                    KeyCode::Right => cursor_right(app),
                    KeyCode::Left => cursor_left(app),
                    _ => {}
                }
            }
        }
    }
}

pub fn update(app: &mut App, key_event: KeyEvent) {
    match app.current_mode {
        crate::app::AppMode::Navigation => {
            match key_event.code {
                KeyCode::Esc => app.quit(),
                KeyCode::Right | KeyCode::Left | KeyCode::Down | KeyCode::Up => {
                    arrow_helper(app, key_event)
                }
                KeyCode::Char('z') => app.undo(),
                KeyCode::Char('I') => app.insert_row(app.selected_row + 1),
                KeyCode::Char('U') => app.insert_col(app.selected_col + 1),
                KeyCode::Enter => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        single_select(app)
                    } else {
                        edit(app)
                    }
                }
                _ => {}
            };
        }
        crate::app::AppMode::Editing => {
            match key_event.code {
                KeyCode::Right | KeyCode::Left => arrow_helper(app, key_event),
                KeyCode::Esc => app.quit_mode(),
                KeyCode::Enter => submit_changes(app),
                KeyCode::Backspace => del_char(app),
                KeyCode::Char(n) => enter_char(app, n),
                _ => {}
            };
        }
        crate::app::AppMode::Selecting => {
            match key_event.code {
                KeyCode::Right | KeyCode::Left | KeyCode::Down | KeyCode::Up => arrow_helper(app, key_event),
                KeyCode::Esc => app.quit_mode(),
                _ => {}
            };
        }
        crate::app::AppMode::SingleSelect => {
            match key_event.code {
                KeyCode::Right | KeyCode::Left | KeyCode::Down | KeyCode::Up => arrow_helper(app, key_event),
                KeyCode::Esc => app.quit_mode(),
                KeyCode::Enter => single_select(app),
                _ => {}
            };
        }
        crate::app::AppMode::Formula => {
            match key_event.code {
                KeyCode::Right | KeyCode::Left => arrow_helper(app, key_event),
                KeyCode::Esc => app.quit_mode(),
                KeyCode::Enter => todo!(),
                KeyCode::Backspace => del_char(app),
                KeyCode::Char(n) => enter_char(app, n),
                _ => {}
            };
        }
    }
}


// pub fn resize(app: &mut App, width: u16, height: u16) {
//   app.selected_row = 0; 
//   app.selected_col = 0;
//   app.view_bound = (0,0);

//   let width = width - 2;
//   let height = height  - 4;

//   app.grid[1][0].content = width.to_string() + ":" + &height.to_string();


//   let mut desired_width = (width / CELL_WIDTH) as usize;
//   let desired_height = height as usize;

//   // app.cell_amount = (desired_width, desired_height);


//   // Handle extra column for remainder
//   let has_remainder = width % CELL_WIDTH != 0;

//   // Adjust the number of rows
//   if app.grid.len() < desired_height {
//       // Add new rows
//       let additional_rows = desired_height - app.grid.len();
//       for _ in 0..additional_rows {
//           app.grid.push(vec![Cell::default(); desired_width]);
//       }
//   }

//   // Adjust the number of columns for each row
//   for row in app.grid.iter_mut() {
//       if has_remainder {
//           // Add an extra column for the remainder
//           row.push(Cell::default());
//           desired_width += 1;
//       }

//       if row.len() < desired_width {
//           // Add new cells to the row
//           let additional_cells = desired_width - row.len();
//           for _ in 0..additional_cells {
//               row.push(Cell::default());
//           }
//       }
//   }


// }