use cursive::Cursive;
use cursive::{
    theme::{BorderStyle, Palette},
    traits::*,
    view::Resizable,
    views::{Dialog, LinearLayout, TextView, ListView, DummyView, Button},
};



fn main() {
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

    let rows = ListView::new()
        .with(|row| {
            for i in 1..27{
                row.add_child( &format!("{i}"),
                    LinearLayout::horizontal()
                        .with(|row|{
                            for y in 1..27 {
                                row.add_child(
                                    LinearLayout::horizontal()
                                    .child(Button::new_raw(&format!("TEST"), Cursive::quit).fixed_width(3))
                                    .child(DummyView)
                                )
                            }
                        })
                );
            }    
        });


    cursive.add_layer(
        Dialog::new()
            .title("exL Sheet")
            .button("Save", |s| s.quit())
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