use crate::{App, app::DEFAULT_COLS};
use std::vec;
use ratatui::{prelude::*, widgets::*};
use crate::tui::Frame;

pub fn render(app: &mut App, f: &mut Frame) {

    let container = Block::default()
        .borders(Borders::ALL)
        .title("exl")
        .title_alignment(Alignment::Center);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(0),
                Constraint::Length(2),
            ]
            .as_ref(),
        )
        .split(container.inner(f.size())); 

    let menu_layout = Layout::default()
    	.direction(Direction::Horizontal)
    	.constraints(
    		[
                Constraint::Min(23),
    			Constraint::Percentage(70),
    			Constraint::Min(23),
    		]
    	)
    	.split(layout[1]);	

    let mut rows: Vec<Row> = Vec::new();

    for (row_index, row_data) in app.grid.iter().enumerate() {
        let mut cells = Vec::new();

        for (col_index, cell) in row_data.iter().enumerate() {

            let cell_style = if row_index == app.selected_row && col_index == app.selected_col {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };

            cells.push(Cell::from(cell.clone()).style(cell_style));
        }

        rows.push(Row::new(cells));
    }

    // let width: Vec<Constraint> = vec![Constraint::Percentage(100 / app.grid[0].len() as u16); app.grid[0].len()];
    let width: Vec<Constraint> = vec![Constraint::Percentage(100 / DEFAULT_COLS as u16); DEFAULT_COLS];

    let table = Table::new(rows)
        .widths(&width);


    f.render_widget(container, f.size());

    f.render_widget(table,layout[0]);

    if let crate::app::AppMode::Editing = app.current_mode {
        f.set_cursor(menu_layout[1].x + app.cursor_pos as u16, menu_layout[1].y + 1);
    }

    f.render_widget(
        match app.current_mode {
            crate::app::AppMode::Navigation => Paragraph::new("-- NAVIGATING --").set_style(Style::default().fg(Color::Green)),
            crate::app::AppMode::Editing => Paragraph::new("-- EDITING --").set_style(Style::default().fg(Color::Yellow)),
            crate::app::AppMode::Selecting => Paragraph::new("-- SELECTING --").set_style(Style::default().fg(Color::Blue)),
        }
        .block(
            Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(Color::Reset))
        ),
        menu_layout[0],
    );

    f.render_widget(
        match app.current_mode {
            crate::app::AppMode::Navigation | crate::app::AppMode::Selecting=> Paragraph::new(app.grid[app.selected_row][app.selected_col].as_str()),
            crate::app::AppMode::Editing => Paragraph::new(app.input.to_owned()).set_style(Style::default().fg(Color::Yellow))
        }
        .block(
            Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(Color::Reset))
        ),
        menu_layout[1],
    );

    f.render_widget(
        Paragraph::new("Q - Menu | ESC - Exit")
        .block(
        	Block::default()
        	.borders(Borders::TOP)
        ).alignment(Alignment::Right),
       	menu_layout[2],
    );
}