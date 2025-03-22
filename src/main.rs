use ratatui_templates::app::{AppResult};
use ratatui_templates::event::{Event, EventsPublisher};
use ratatui_templates::handler::handle_key_events;
use ratatui_templates::connection::get_data;
use ratatui_templates::tui::Tui;
use std::{io, error::Error};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

// Import directly from the module


fn main() -> AppResult<()> {
    // Setup the terminal
    match get_data("London".to_string()) {
        Ok(city_data) => {
            println!("{:?}", city_data);
            Ok(())
        },
        Err(e) => {
            eprintln!("Failed to fetch city data: {}", e);
            Err(Box::new(io::Error::new(io::ErrorKind::Other, e.to_string())))
        }
    }

    // ...existing code...
}
