use crate::app::ArrowKeys;
use crate::app::App;
use crate::app::AppMode;
use crate::app::MinMaxRange;



pub fn select_cell(app: &mut App, row: usize, col: usize) {
    app.selected_row = row;
    app.selected_col = col;
}


pub fn single_select(app: &mut App) {
    app.current_mode = AppMode::SingleSelect;
    app.grid[app.selected_row][app.selected_col].selected = true;

}

pub fn select(app: &mut App) {
    app.current_mode = AppMode::Selecting;

    app.grid[app.selected_row][app.selected_col].selected = true;

    //init the selected ranges
    app.selected_range = Some(MinMaxRange { 
        min_x: app.selected_row,
        max_x: app.selected_row,
        min_y: app.selected_col,
        max_y: app.selected_col 
    });

}

pub fn select_nav(app: &mut App, direction: ArrowKeys) {
    app.nav(direction);
    app.grid[app.selected_row][app.selected_col].selected = true;

    let mut range = app.selected_range.as_mut().unwrap();

    // Update ranges after new cell is selected
    app.selected_range = Some(MinMaxRange { 
        min_x: range.min_x.min(app.selected_row),
        max_x: range.max_x.max(app.selected_row),
        min_y: range.min_y.min(app.selected_col),
        max_y: range.max_y.max(app.selected_col) 
    });

    range = app.selected_range.as_mut().unwrap();


    // Traverse the grid in the selected range flip all those cells as selected
    for row in &mut app.grid[range.min_x..=range.max_x] {

        for cell in &mut row[range.min_y..=range.max_y] {
            cell.selected = true;
        }

    }

}
