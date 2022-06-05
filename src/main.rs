use std::io;
use tui::{Terminal, backend::CrosstermBackend}; 

use rbudget::errors::Error;

extern crate log;
extern crate pretty_env_logger;

fn main() -> Result<(), Error> {
    pretty_env_logger::init();
    
    // Init tui application
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    Ok(())
}



