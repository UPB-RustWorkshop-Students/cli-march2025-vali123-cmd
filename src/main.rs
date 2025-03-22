use ratatui_templates::app::{App, AppResult};
use ratatui_templates::event::{Event, EventsPublisher};
use ratatui_templates::handler::handle_key_events;
use ratatui_templates::tui::Tui;
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use ratatui_templates::connection::get_data;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    // let app =

    // Setup the terminal
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let city_name = "Focsani";
    match get_data(city_name.to_string()) {
        Ok(city_info) => println!("{:?}", city_info),
        Err(e) => eprintln!("Error: {}", e),
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