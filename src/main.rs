// INSPIRATION: https://github.com/fdehau/tui-rs/blob/master/examples/user_input.rs

use std::io;

use tui::{
    Terminal, Frame, 
    backend::{Backend, CrosstermBackend},
    widgets::{Block, Borders, ListItem, List},
    layout::{Layout, Constraint, Direction}, 
    text::{Span},
    style::{Style, Color}
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

struct Controller {
    datastore: DataStore, 
    state: State    
}

#[derive(PartialEq)]
enum Field {
    Name, Tag, Date, Amount
}

enum Mode {
    Normal,
    Insert(Field) 
}

struct State {
    input: String, 
    buffer: Expense,
    mode: Mode
}

impl Controller {
    fn new() -> Result<Controller, Error> {
        Ok(Controller {
            datastore: DataStore::new()?, 
            state: State {
                input: String::new(),
                buffer: Expense::empty(),
                mode: Mode::Normal
            }
        })
    }

    fn exec(&mut self, cmd: char) {
        match cmd {
            'i' => self.state.mode = Mode::Insert(Field::Name),
            _ => {}
        }
    }
}


fn main() -> Result<(), Error> {
   
    // Initialization 
    pretty_env_logger::init();
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let app = Controller::new()?;
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

    info!("Finished Execution");
    Ok(())
}

fn run<B: Backend>(terminal: &mut Terminal<B>, mut app: Controller) -> Result<(), Error> {
    loop {
        terminal.draw(|f| render(f, &app))?;

        let event = crossterm::event::read()?;
        match app.state.mode {
            Mode::Normal => {
                match event {
                    Event::Key(key) => {
                        match key.code {
                            KeyCode::Char(ch) => {
                                app.exec(ch);
                            },
                            KeyCode::Esc => {
                                return Ok(());
                            }
                            _ => {}
                        }
                    },
                    _ => {}
                }
            },
            Mode::Insert(_) => {
                match event {
                    Event::Key(key) => {
                        match key.code {
                            KeyCode::Char(ch) => {
                                app.state.input.push(ch);
                            },
                            KeyCode::Enter | KeyCode::Tab => {
                                let rec = Expense::new(uuid::Uuid::new_v4().to_bytes_le(), app.state.input.clone(), app.state.input.clone(), chrono::Local::today().and_hms(0, 0, 0).timestamp(), 100);
                                app.datastore.append_one(&rec)?;
                                app.state.input = String::new();
                            },
                            KeyCode::Esc => {
                                app.state.mode = Mode::Normal;
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            }
        }
    }
}

fn render<B: Backend>(frame: &mut Frame<B>, app: &Controller) {
   
    // Top & Bottom
    let stack = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(12),
                Constraint::Percentage(90)
            ].as_ref()
        )
        .split(frame.size()); 

    // Divide top into 3 columns
    let chunk = Layout::default()
        .direction(Direction::Horizontal) 
        .margin(2)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(40),
                Constraint::Percentage(40)
            ].as_ref()
        )
        .split(stack[0]);

    // Input fields and blocks
    let input_stack_layout = Layout::default()
        .direction(Direction::Vertical) 
        .margin(1) 
        .constraints(
            [Constraint::Length(3), Constraint::Length(3)].as_ref()
        );
    let input_stack_left = input_stack_layout.split(chunk[1]);
    let input_stack_right = input_stack_layout.split(chunk[2]);

    let instruction = Span::from("Press i to insert expense");
    let instruction_block = Block::default()
        .borders(Borders::ALL)
        .title(vec![
            Span::from("Help \n"),
            instruction
        ]);

    // Name Input 
    let input_block = render_input_block(Field::Name);
    let input_block_inner_area = input_block.inner(input_stack_left[0]);
    frame.render_widget(input_block, input_stack_left[0]);
    let block_content = render_input(app, Field::Name);
    frame.render_widget(Block::default().title(block_content), input_block_inner_area);

    // Tag Input 
    let input_block = render_input_block(Field::Tag);
    let input_block_inner_area = input_block.inner(input_stack_left[1]);
    frame.render_widget(input_block, input_stack_left[1]);
    let block_content = render_input(app, Field::Tag);
    frame.render_widget(Block::default().title(block_content), input_block_inner_area);

    // Date Input 
    let input_block = render_input_block(Field::Date);
    let input_block_inner_area = input_block.inner(input_stack_right[0]);
    frame.render_widget(input_block, input_stack_right[0]);
    let block_content = render_input(app, Field::Date);
    frame.render_widget(Block::default().title(block_content), input_block_inner_area);

    // Amount Input
    let input_block = render_input_block(Field::Amount);
    let input_block_inner_area = input_block.inner(input_stack_right[1]);
    frame.render_widget(input_block, input_stack_right[1]);
    let block_content = render_input(app, Field::Amount);
    frame.render_widget(Block::default().title(block_content), input_block_inner_area);

    frame.render_widget(instruction_block, chunk[0]);
    frame.render_widget(render_list(app), stack[1]);
}

// UI Component Render 
fn render_list(app: &Controller) -> List {
    let data = app.datastore.list_all().unwrap();
    let mut ul = Vec::new();
    for record in data {
        let li = ListItem::new(record.name()); 
        ul.push(li);
    }
    return List::new(ul)
        .block(Block::default().title("Expenses").borders(Borders::ALL));
}

fn render_input<'a>(app: &Controller, field: Field) -> Span<'a> {
    if let Mode::Insert(f) = &app.state.mode {
        if &field == f {
            return Span::from(app.state.input.clone());
        }
    }
    return Span::from("");
}

fn render_input_block<'a>(field: Field) -> Block<'a> {
    let name = match field { 
        Field::Name => "Title",
        Field::Tag => "Tag",
        Field::Amount => "Amount",
        Field::Date => "Date"
    };

    Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(name, Style::default().fg(Color::Yellow)))
}
