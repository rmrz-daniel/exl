use crate::app::{App, AppMode};
use crate::modes::edit::*;
use crate::modes::select::*;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::todo;

fn arrow_helper(app: &mut App, key_event: KeyEvent) {
    if key_event.modifiers == KeyModifiers::SHIFT {
        select(app)
    } else if key_event.modifiers == KeyModifiers::CONTROL {
        match key_event.code {
            KeyCode::Left | KeyCode::Char('h') => app.insert_col(app.selected_col),
            KeyCode::Right | KeyCode::Char('l') => app.insert_col(app.selected_col + 1),
            KeyCode::Up | KeyCode::Char('k') => app.insert_row(app.selected_row),
            KeyCode::Down | KeyCode::Char('j') => app.insert_row(app.selected_row + 1),
            _ => {}
        }
    } else {
        match app.current_mode {
            AppMode::Navigation | AppMode::Formula | AppMode::SingleSelect => {
                match key_event.code {
                    KeyCode::Right | KeyCode::Char('l') => app.nav(crate::app::ArrowKeys::Right),
                    KeyCode::Left | KeyCode::Char('h') => app.nav(crate::app::ArrowKeys::Left),
                    KeyCode::Down | KeyCode::Char('j') => app.nav(crate::app::ArrowKeys::Down),
                    KeyCode::Up | KeyCode::Char('k') => app.nav(crate::app::ArrowKeys::Up),
                    _ => {}
                }
            }
            AppMode::Selecting => match key_event.code {
                KeyCode::Right | KeyCode::Char('l') => {
                    select_nav(app, crate::app::ArrowKeys::Right)
                }
                KeyCode::Left | KeyCode::Char('h') => select_nav(app, crate::app::ArrowKeys::Left),
                KeyCode::Down | KeyCode::Char('j') => select_nav(app, crate::app::ArrowKeys::Down),
                KeyCode::Up | KeyCode::Char('k') => select_nav(app, crate::app::ArrowKeys::Up),
                _ => {}
            },
            AppMode::Editing => match key_event.code {
                KeyCode::Right => cursor_right(app),
                KeyCode::Left => cursor_left(app),
                _ => {}
            },
        }
    }
}

pub fn update(app: &mut App, key_event: KeyEvent) {
    match app.current_mode {
        AppMode::Navigation => {
            match key_event.code {
                KeyCode::Esc => app.quit(),
                KeyCode::Right
                | KeyCode::Left
                | KeyCode::Down
                | KeyCode::Up
                | KeyCode::Char('h')
                | KeyCode::Char('j')
                | KeyCode::Char('k')
                | KeyCode::Char('l') => arrow_helper(app, key_event),
                KeyCode::Backspace | KeyCode::Char('x') => app.delete(),
                KeyCode::Char('z') | KeyCode::Char('u') => app.undo(),
                KeyCode::Enter | KeyCode::Char('i') | KeyCode::Char('a') => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        single_select(app)
                    } else {
                        edit(app)
                    }
                }
                _ => {}
            };
        }
        AppMode::Editing => {
            match key_event.code {
                KeyCode::Right | KeyCode::Left => arrow_helper(app, key_event),
                KeyCode::Esc => app.quit_mode(),
                KeyCode::Enter => submit_changes(app),
                KeyCode::Backspace => del_char(app),
                KeyCode::Char(n) => enter_char(app, n),
                _ => {}
            };
        }
        AppMode::Selecting => {
            match key_event.code {
                KeyCode::Right
                | KeyCode::Left
                | KeyCode::Down
                | KeyCode::Up
                | KeyCode::Char('h')
                | KeyCode::Char('j')
                | KeyCode::Char('k')
                | KeyCode::Char('l') => arrow_helper(app, key_event),
                KeyCode::Esc => app.quit_mode(),
                _ => {}
            };
        }
        AppMode::SingleSelect => {
            match key_event.code {
                KeyCode::Right
                | KeyCode::Left
                | KeyCode::Down
                | KeyCode::Up
                | KeyCode::Char('h')
                | KeyCode::Char('j')
                | KeyCode::Char('k')
                | KeyCode::Char('l') => arrow_helper(app, key_event),
                KeyCode::Esc => app.quit_mode(),
                KeyCode::Enter | KeyCode::Char('i') | KeyCode::Char('a') => single_select(app),
                _ => {}
            };
        }
        AppMode::Formula => {
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
