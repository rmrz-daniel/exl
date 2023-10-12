use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::modes::edit::*;
use crate::modes::select::*;
use crate::app::App;

fn arrow_helper(app: &mut App, key_event: KeyEvent) {
    if key_event.modifiers == KeyModifiers::SHIFT {
        select(app)
    } else {
        match app.current_mode {
            crate::app::AppMode::Navigation
            | crate::app::AppMode::Formula
            | crate::app::AppMode::SingleSelect => match key_event.code {
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
            crate::app::AppMode::Editing | crate::app::AppMode::FormulaInput => {
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
                KeyCode::Esc => app.quit_mode(),
                KeyCode::Right | KeyCode::Left => arrow_helper(app, key_event),
                KeyCode::Enter => submit_changes(app),
                KeyCode::Backspace => del_char(app),
                KeyCode::Char(n) => enter_char(app, n),
                _ => {}
            };
        }
        crate::app::AppMode::Selecting => {
            match key_event.code {
                KeyCode::Esc => app.quit_mode(),
                KeyCode::Right | KeyCode::Left | KeyCode::Down | KeyCode::Up => {
                    arrow_helper(app, key_event)
                }
                KeyCode::Char('F') => app.formula(),
                _ => {}
            };
        }
        crate::app::AppMode::SingleSelect => {
            match key_event.code {
                KeyCode::Esc => app.quit_mode(),
                KeyCode::Enter => single_select(app),
                KeyCode::Right | KeyCode::Left | KeyCode::Down | KeyCode::Up => {
                    arrow_helper(app, key_event)
                }
                KeyCode::Char('F') => app.formula(),
                _ => {}
            };
        }
        crate::app::AppMode::Formula => {
            match key_event.code {
                KeyCode::Esc => app.quit_mode(),
                KeyCode::Right | KeyCode::Left | KeyCode::Down | KeyCode::Up => {
                    arrow_helper(app, key_event)
                }
                KeyCode::Enter => app.formula_input(),
                _ => {}
            };
        }
        crate::app::AppMode::FormulaInput => {
            match key_event.code {
                KeyCode::Esc => app.quit_mode(),
                KeyCode::Right | KeyCode::Left => arrow_helper(app, key_event),
                KeyCode::Enter => submit_changes(app),
                KeyCode::Backspace => del_char(app),
                KeyCode::Char(n) => enter_char(app, n),
                _ => {}
            };
        }
    }
}
