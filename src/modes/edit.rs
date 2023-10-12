use crate::app::GridState;
use crate::app::App;
use crate::app::AppMode;


const MAX_UNDO_LEVELS: usize = 7;

pub fn edit(app: &mut App) {
    app.input = app.grid[app.selected_row][app.selected_col].content.clone();
    app.current_mode = AppMode::Editing;
    app.cursor_pos = app.input.len();
}

pub fn cursor_left(app: &mut App) {
    app.cursor_pos = clamp(app, app.cursor_pos.saturating_sub(1))
}

pub fn cursor_right(app: &mut App) {
    app.cursor_pos = clamp(app, app.cursor_pos.saturating_add(1))
}

fn clamp(app: &mut App, new_pos: usize) -> usize {
    new_pos.clamp(0, app.input.len())
}

pub fn enter_char(app: &mut App, new_char: char) {
    app.input.insert(app.cursor_pos, new_char);
    cursor_right(app);
}

pub fn del_char(app: &mut App) {
    if app.cursor_pos != 0 {
        let left_of_del = app.input.chars().take(app.cursor_pos - 1);
        let right_of_del = app.input.chars().skip(app.cursor_pos);

        app.input = left_of_del.chain(right_of_del).collect();
        cursor_left(app);
    }
}

pub fn submit_changes(app: &mut App) {
    let cloned_grid = GridState {
        grid: app.grid.clone(),
    };

    app.grid[app.selected_row][app.selected_col].content = app.input.to_string();
    app.cursor_pos = 0;
    app.quit_mode();

    if app.undo_stack.len() >= MAX_UNDO_LEVELS {
        app.undo_stack.remove(0);
    }

    app.undo_stack.push(cloned_grid);
}
