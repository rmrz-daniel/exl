use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::{App};


fn arrow_helper(app: &mut App, key_event: KeyEvent) {

  if key_event.modifiers == KeyModifiers::SHIFT {
    
    app.select()

  } else {
    match app.current_mode {
      crate::app::AppMode::Navigation | crate::app::AppMode::Formula | crate::app::AppMode::SingleSelect  => {
        match key_event.code {
          KeyCode::Right => app.nav(crate::app::ArrowKeys::Right),
          KeyCode::Left => app.nav(crate::app::ArrowKeys::Left),
          KeyCode::Down => app.nav(crate::app::ArrowKeys::Down),
          KeyCode::Up => app.nav(crate::app::ArrowKeys::Up),
          _ => {},
        }
      },
      crate::app::AppMode::Selecting => {
        match key_event.code {
          KeyCode::Right => app.select_nav(crate::app::ArrowKeys::Right),
          KeyCode::Left => app.select_nav(crate::app::ArrowKeys::Left),
          KeyCode::Down => app.select_nav(crate::app::ArrowKeys::Down),
          KeyCode::Up => app.select_nav(crate::app::ArrowKeys::Up),
          _ => {},
        }
      },
      crate::app::AppMode::Editing | crate::app::AppMode::FormulaInput => {
        match key_event.code {
          KeyCode::Right => app.cursor_right(),
          KeyCode::Left => app.cursor_left(),
          _ => {},
        }
      },
    }
  }
}

pub fn update(app: &mut App, key_event: KeyEvent) {

  match app.current_mode {
    crate::app::AppMode::Navigation => {
      match key_event.code {
        KeyCode::Esc => app.quit(),
        KeyCode::Right | KeyCode::Left | KeyCode::Down | KeyCode::Up => arrow_helper(app, key_event),
        KeyCode::Char('z') => app.undo(),
        KeyCode::Enter => {
          if key_event.modifiers == KeyModifiers::CONTROL {
            app.singel_select()
          } else {
            app.edit()
          }
        },
        _ => {},
      };
    },
    crate::app::AppMode::Editing => {
      match key_event.code {
        KeyCode::Esc => app.quit_mode(),
        KeyCode::Right | KeyCode::Left => arrow_helper(app, key_event),
        KeyCode::Enter => app.submit_changes(),
        KeyCode::Backspace => app.del_char(),
        KeyCode::Char(n) => app.enter_char(n),
        _ => {},
      };
    },
    crate::app::AppMode::Selecting => {
      match key_event.code {
        KeyCode::Esc => app.quit_mode(),
        KeyCode::Right | KeyCode::Left | KeyCode::Down | KeyCode::Up => arrow_helper(app, key_event),
        KeyCode::Char('F') => app.formula(),
        _ => {},
      };
    },
    crate::app::AppMode::SingleSelect => {
      match key_event.code {
        KeyCode::Esc => app.quit_mode(),
        KeyCode::Enter => app.singel_select(),
        KeyCode::Right | KeyCode::Left | KeyCode::Down | KeyCode::Up => arrow_helper(app, key_event),
        KeyCode::Char('F') => app.formula(),
        _ => {},
      };
    },
    crate::app::AppMode::Formula => {
      match key_event.code {
        KeyCode::Esc => app.quit_mode(),
        KeyCode::Right | KeyCode::Left | KeyCode::Down | KeyCode::Up => arrow_helper(app, key_event),
        KeyCode::Enter => app.formula_input(),
        _ => {},
      };
    },
    crate::app::AppMode::FormulaInput => {
      match key_event.code {
        KeyCode::Esc => app.quit_mode(),
        KeyCode::Right | KeyCode::Left => arrow_helper(app, key_event),
        KeyCode::Enter => app.submit_changes(),
        KeyCode::Backspace => app.del_char(),
        KeyCode::Char(n) => app.enter_char(n),
        _ => {},
      };
    }
  }
}