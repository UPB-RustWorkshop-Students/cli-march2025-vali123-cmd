use ratatui_templates::app::{App, AppResult};
use ratatui_templates::event::{Event, EventsPublisher};
use ratatui_templates::handler::handle_key_events;
use ratatui_templates::tui::Tui;
use std::{io, error::Error};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use ratatui_templates::connection::get_data;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Setup the terminal
    match get_data("London".to_string()) {
        Ok(city_data) => println!("{:?}", city_data),
        Err(e) => {
            eprintln!("Failed to fetch city data: {}", e);
            return Err(Box::new(e)); // Propagate the error
        }
    }

    // TODO: create the events publisher
    // let events_publisher= ...

    // TODO: init the terminal user interface
    // let mut tui =

    // Start the main loop.
    // while app.running {
    // TODO: Render the user interface.

    // TODO: Handle events.
    // Hint: wait for events and handle them

    // }

    // TODO: Reset the terminal if the app has been terminated

    Ok(())
}