pub mod app;
pub mod event;
pub mod ui;
pub mod tui;
pub mod update;

use anyhow::Result;
use app::App;
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;
use update::update;

fn main() -> Result<()> {
  let mut app = App::new();

  // Initialize the terminal user interface.
  let backend = CrosstermBackend::new(std::io::stderr());
  let terminal = Terminal::new(backend)?;
  let events = EventHandler::new(250);
  let mut tui = Tui::new(terminal, events);
  tui.init()?;

  while !app.should_quit {
    // Render the user interface.
    tui.draw(&mut app)?;
    // Handle events.
    match tui.events.next()? {
      Event::Tick => {},
      Event::Key(key_event) => update(&mut app, key_event),
      Event::Resize(_, _) => {},
    };
  }

  tui.exit()?;
  Ok(())
}