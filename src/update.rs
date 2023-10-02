use std::todo;

use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App};

pub fn update(app: &mut App, key_event: KeyEvent) {

  match app.current_mode {
    crate::app::AppMode::Navigation => {
        match key_event.code {
          KeyCode::Esc => app.quit(),
          KeyCode::Right => app.nav_right(),
          KeyCode::Left => app.nav_left(),
          KeyCode::Down => app.nav_down(),
          KeyCode::Up => app.nav_up(),
          KeyCode::Enter => app.edit(),
          _ => {},
        };
    },
    crate::app::AppMode::Editing => {
        match key_event.code {
          KeyCode::Esc => app.quit_mode(),
          KeyCode::Right => app.cursor_right(),
          KeyCode::Left => app.cursor_left(),
          KeyCode::Enter => app.submit_changes(),
          KeyCode::Backspace => app.del_char(),
          KeyCode::Char(n) => app.enter_char(n),
          _ => {},
        };
    },
    crate::app::AppMode::Selecting => todo!(),
}
}