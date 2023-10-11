use crate::app::ArrowKeys;
use crate::app::App;
use crate::app::AppMode;

pub fn select_cell(app: &mut App, row: usize, col: usize) {
    app.selected_row = row;
    app.selected_col = col;
}

pub fn select(app: &mut App) {
    app.current_mode = AppMode::Selecting;

    // cell_map.insert(
    //     (app.selected_row, app.selected_col),
    //     app.grid[app.selected_row][app.selected_col].clone(),
    // );

}

pub fn singel_select(app: &mut App) {
    app.current_mode = AppMode::SingleSelect;
    app.grid[app.selected_row][app.selected_col].selected = true;

}

pub fn select_nav(app: &mut App, direction: ArrowKeys) {
    app.nav(direction);

    // app.selected_cells.as_mut().unwrap().insert(
    //     (app.selected_row, app.selected_col),
    //     app.grid[app.selected_row][app.selected_col].clone(),
    // );

    // let (min_x, max_x, min_y, max_y) = app
    //     .selected_cells
    //     .as_ref()
    //     .unwrap()
    //     .iter()
    //     .fold(None, |acc, ((x, y), _)| {
    //         match acc {
    //             None => Some((*x, *x, *y, *y)), //initialize to the first pair
    //             Some((min_x, max_x, min_y, max_y)) => {
    //                 Some((min_x.min(*x), max_x.max(*x), min_y.min(*y), max_y.max(*y)))
    //             }
    //         }
    //     })
    //     .unwrap();

    // for (row, row_vec) in app.grid.iter().enumerate() {
    //     for (col, content) in row_vec.iter().enumerate() {
    //         if !app
    //             .selected_cells
    //             .as_ref()
    //             .unwrap()
    //             .contains_key(&(row, col))
    //             && row >= min_x
    //             && row <= max_x
    //             && col >= min_y
    //             && col <= max_y
    //         {
    //             app.selected_cells
    //                 .as_mut()
    //                 .unwrap()
    //                 .insert((row, col), content.clone());
    //         }
    //     }
    // }
}
