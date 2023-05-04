use csv::StringRecord;
use std::env::args;
use std::fs::File;
use cursive::Cursive;
use cursive::views::EditView;
use cursive::{
    theme::{BorderStyle, Palette},
    traits::*,
    view::Resizable,
    views::{Dialog, LinearLayout, TextView, ListView, DummyView, Button},
};

use crate::Arguments;

pub fn run(args: Option<Arguments>) {
	 //start Cursive root
    let mut cursive = cursive::default();

    //Set theme to the default terminal theme on system.
    cursive.set_theme(cursive::theme::Theme {
        shadow: false,
        borders: BorderStyle::Simple,
        palette: Palette::terminal_default()
    });

    let col_labels = LinearLayout::horizontal()
        .child(DummyView.fixed_width(4))
        .with(|col| {
            for i in 65u8..91 {
                col.add_child(
                    LinearLayout::horizontal()
                    .child(TextView::new(i as char).fixed_width(3))
                    .child(DummyView)
                );
            }
        });

    let rows = match args {
    	Some(args) => create_rows_with_file(args),
    	None => create_rows()
    };    

    cursive.add_global_callback('q', |s| s.quit());
    cursive.add_global_callback('s', save);
    cursive.add_layer(
        Dialog::new()
            .title("Quick exL")
            .button("Save", save)
            .button("Quit", |s| s.quit())
            .content(
                LinearLayout::vertical()
                .child(col_labels)
                .child(rows)
            )
            .fixed_width(110)
    );


    cursive.run();
}

fn create_rows() -> ListView {
	ListView::new()
    .with(|row| {
        for i in 1..27{
            row.add_child( &format!("{i}"),
                LinearLayout::horizontal()
                    .with(|row|{
                        for y in 1..27 {
                            let c = y.clone();
                            let r = i.clone();
                            row.add_child(
                                LinearLayout::horizontal()
                                .child(Button::new_raw(" ", move |cursive| {edit_cell(cursive, r, c)}).with_name(format!("{r},{c}")).fixed_width(3))
                                .child(DummyView)
                            )
                        }
                    })
            );
        }    
    })    
}

fn create_rows_with_file(args: Arguments) -> ListView {
	let file: File = File::open(args.file).expect("File does not exist / could not be found");
	let mut csv_reader = csv::ReaderBuilder::new()
		.has_headers(false)
		.from_reader(file);
	let rows: Vec<StringRecord> = csv_reader.records().flatten().collect();

	ListView::new()
    .with(|row| {
        for i in 1..27{
            row.add_child( &format!("{i}"),
                LinearLayout::horizontal()
                    .with(|row|{
                        for y in 1..27 {
                            let c = y.clone();
                            let r = i.clone();
                            row.add_child(
                                LinearLayout::horizontal()
                                .child(Button::new_raw(rows[i - 1][y - 1].to_string().clone(), move |cursive| {edit_cell(cursive, r as i32, c as i32)}).with_name(format!("{r},{c}")).fixed_width(3))
                                .child(DummyView)
                            )
                        }
                    })
            );
        }    
    })   
}

fn edit_cell(s: &mut Cursive, row: i32, col: i32) {

    fn ok(s: &mut Cursive, new_cell: &str, row: i32, col: i32) {
        s.call_on_name(&format!("{row},{col}"), |view: &mut Button| {
            view.set_label_raw(new_cell)
        });

        s.pop_layer();
    }

    let mut cell_content = String::new();

    s.call_on_name(&format!("{row},{col}"), |view: &mut Button| {
        cell_content = String::from(view.label())
    });

    s.add_layer(
        Dialog::new()
        .content(
            EditView::new()
            .content(cell_content)
            .on_submit(move |s, content| {

                ok(s, content, row, col)
            })
            .with_name("cell")
            .fixed_width(20)

        )
        // .button("Ok", move |x|{
        //     let new_cell_content = x.call_on_name("cell", |view: &mut EditView| {
        //         view.get_content()
        //     }).unwrap();

        //     ok(x, &new_cell_content, row, col)
        // })
        .button("Cancel", |x| { x.pop_layer(); })
    )
}

fn save(s: &mut Cursive) {

	let layer = match args().nth(1) {
		Some(filepath) => {
	        Dialog::new()
	        .title("Save as")
	        .content(
	            LinearLayout::vertical()
	            .child(TextView::new("File Path:"))
	            .child(
	                EditView::new()
	                .content(filepath.clone()[..filepath.find('.').unwrap()].to_string())
	                .on_submit(|s, filepath| ok(s, filepath.to_string()))
	                .with_name("file_path")
	                .fixed_width(40)
	            )
	        )
	        .button("Save", move |s| {
	            ok(s, filepath.clone());
	        })
	        .button("save as", |s| {
	            let filepath = s.call_on_name("file_path", |view: &mut EditView| view.get_content()).unwrap();
	            ok(s, filepath.to_string() + ".csv");
	        })
	        .button("Cancel", |s| { s.pop_layer(); })	
		},
		None => {
	        Dialog::new()
	        .title("Save as")
	        .content(
	            LinearLayout::vertical()
	            .child(TextView::new("File Path:"))
	            .child(
	                EditView::new()
	                .on_submit(|s, filepath| ok(s, filepath.to_string()))
	                .with_name("file_path")
	                .fixed_width(40)
	            )
	        )
	        .button("save", |s| {
	            let filepath = s.call_on_name("file_path", |view: &mut EditView| view.get_content()).unwrap();
	            ok(s, filepath.to_string() + ".csv");
	        })
	        .button("Cancel", |s| { s.pop_layer(); })
		}
	};

    s.add_layer(layer);

    fn ok(s: &mut Cursive, file_path: String){
        let mut content_array: Vec<Vec<String>> = Vec::new();

        for row in 1..27 {
            let mut row_vec: Vec<String> = Vec::new();
            for col in 1..27 {
                let mut cell_content = String::new();

                s.call_on_name(&format!("{row},{col}"), |view: &mut Button| {
                    cell_content = String::from(view.label())
                });

                row_vec.push(cell_content);

                // if cell_content != " " {
                //     row_vec.push(cell_content);
                // }

            }

            if row_vec.is_empty() == false {
                content_array.push(row_vec);
            }
        }

        let mut csv_writer = csv::Writer::from_path(file_path).unwrap();

        for row in content_array {
            csv_writer.write_record(row).unwrap(); 
        }

        s.quit()
    }
}