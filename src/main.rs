// INSPIRATION: https://github.com/fdehau/tui-rs/blob/master/examples/user_input.rs

use std::io;

use tui::{
    Terminal, Frame, 
    backend::{Backend, CrosstermBackend},
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction}, 
}; 

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

extern crate log;
use log::*;
extern crate pretty_env_logger;

use rbudget::errors::Error;
use rbudget::data::*;
use rbudget::record::*;

/// Connects backend, persistency and frontent event 
struct Controller {
    datastore: DataStore, 
    state: State    
}

struct State {
    input: String
}

impl Controller {
    fn new() -> Result<Controller, Error> {
        Ok(Controller {
            datastore: DataStore::new()?, 
            state: State {
                input: String::new() 
            }
        })
    }
}


fn main() -> Result<(), Error> {
   
    // Initialization 
    pretty_env_logger::init();
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut app = Controller::new()?;
    let mut terminal = Terminal::new(backend)?;

    // Start the app
    let app_result = run(&mut terminal, app);
    
    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Log any error seen during execution
    if let Err(error) = app_result {
        error!("{}", error);
    }

    Ok(())
}

fn run<B: Backend>(terminal: &mut Terminal<B>, mut app: Controller) -> Result<(), Error> {
    loop {
        terminal.draw(|f| render(f, &app))?;

        let event = crossterm::event::read()?;
        match event {
            Event::Key(key) => {
                match key.code {
                    KeyCode::Char(ch) => {
                        app.state.input.push(ch); 
                    },
                    KeyCode::Esc => {
                        return Ok(());
                    },
                    KeyCode::Enter => {
                        let exp = Expense::new(uuid::Uuid::new_v4().to_bytes_le(), app.state.input, String::from("test"),chrono::Local::today().and_hms(0,0,0).timestamp(), 100);
                        return app.datastore.append_one(&exp);
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
}

fn render<B: Backend>(frame: &mut Frame<B>, app: &Controller) {
    let grid = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(90)
            ].as_ref()
        )
        .split(frame.size()); 

    let input_block = Block::default()
        .borders(Borders::ALL) 
        .title("Record Expense");
    frame.render_widget(input_block, grid[0]);

    let input_block = Block::default()
        .borders(Borders::ALL) 
        .title("Recent Expenses");
    frame.render_widget(input_block, grid[1]);
}

