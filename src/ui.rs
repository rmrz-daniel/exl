use crate::{App};
use crate::app::Cell as ExlCell;
use std::{vec};
use ratatui::widgets::Cell as RatatuiCell;
use ratatui::{prelude::*, widgets::*};
use crate::tui::Frame;


fn get_style(cell: &ExlCell, row_index: usize, col_index: usize, app: &App) -> Style {
    if cell.selected {
        
        return Style::default().bg(Color::Blue).fg(Color::White).add_modifier(
            if (row_index, col_index ) == (app.selected_row, app.selected_col){
                Modifier::REVERSED
            } else {
                Modifier::empty()
            } 
        );

    } else {
        return Style::default().add_modifier(
            if (row_index, col_index ) == (app.selected_row, app.selected_col){
                Modifier::REVERSED
            } else {
                Modifier::empty()
            }  
        );
    };
}

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
    
    let mut width: Vec<Constraint> = vec![Constraint::Length(12); app.grid[0].len()];
    if layout[0].width & 12 != 0 {
        width.push(Constraint::Length(layout[0].width & 12))
    }


    let mut rows: Vec<Row> = Vec::new();

    for (row_index, row_data) in app.grid.iter().enumerate() {
        let mut cells: Vec<RatatuiCell> = Vec::new();

        for (col_index, cell) in row_data.iter().enumerate() {

            cells.push(RatatuiCell::from(cell.content.clone()).style(get_style(cell, row_index, col_index, app)));
        }

        rows.push(Row::new(cells).bottom_margin(0));
    }

    // let width: Vec<Constraint> = vec![Constraint::Percentage(100 / app.grid[0].len() as u16); app.grid[0].len()];

    let table = Table::new(rows)
    .widths(&width)
    .column_spacing(0);


    f.render_widget(container, f.size());

    f.render_widget(table,layout[0]);

    if let crate::app::AppMode::Editing | crate::app::AppMode::FormulaInput = app.current_mode {
        f.set_cursor(menu_layout[1].x + app.cursor_pos as u16, menu_layout[1].y + 1);
    }

    f.render_widget(
        match app.current_mode {
            crate::app::AppMode::Navigation => Paragraph::new("-- NAVIGATING --").set_style(Style::default().fg(Color::Green)),
            crate::app::AppMode::Editing => Paragraph::new("-- EDITING --").set_style(Style::default().fg(Color::Yellow)),
            crate::app::AppMode::Selecting | crate::app::AppMode::SingleSelect => Paragraph::new("-- SELECTING --").set_style(Style::default().fg(Color::Blue)),
            crate::app::AppMode::Formula | crate::app::AppMode::FormulaInput => Paragraph::new("-- Formula --").set_style(Style::default().fg(Color::Magenta)),
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
            crate::app::AppMode::Navigation => Paragraph::new(app.grid[app.selected_row][app.selected_col].content.as_str()),
            crate::app::AppMode::Editing => Paragraph::new(app.input.to_owned()).set_style(Style::default().fg(Color::Yellow)),
            crate::app::AppMode::FormulaInput => Paragraph::new(app.input.to_owned()).set_style(Style::default().fg(Color::Magenta)), 
            crate::app::AppMode::Selecting | crate::app::AppMode::Formula | crate::app::AppMode::SingleSelect=> {

                Paragraph::new("Temp")
            },
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