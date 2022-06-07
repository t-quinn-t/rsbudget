use std::io;
use std::thread;
use std::time::Duration;

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

use rbudget::errors::Error;

extern crate log;
extern crate pretty_env_logger;

fn main() -> Result<(), Error> {
    pretty_env_logger::init();

    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    // Init tui application
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    terminal.draw(|f| {
        render_layout(f);
    })?;

    // TODO: add quit function
    thread::sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn render_layout<B: Backend>(frame: &mut Frame<B>) {
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

