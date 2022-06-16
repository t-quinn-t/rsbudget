// INSPIRATION: https://github.com/fdehau/tui-rs/blob/master/examples/user_input.rs

use std::io;

use tui::{
    Terminal, Frame, 
    backend::{Backend, CrosstermBackend},
    widgets::{Block, Borders, ListItem, List, Tabs, Table, Cell, Row},
    layout::{Layout, Constraint, Direction}, 
    text::{Span, Spans},
    style::{Style, Color}
}; 

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use chrono::prelude::{Local, TimeZone, Utc};

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
        match &app.state.mode {
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
            Mode::Insert(field) => {
                match event {
                    Event::Key(key) => {
                        match key.code {
                            KeyCode::Char(ch) => {
                                app.state.input.push(ch);
                            },
                            KeyCode::Enter | KeyCode::Tab => {
                                let input_val = app.state.input;
                                match field {
                                    Field::Name => {
                                        app.state.buffer.set_name(&input_val);
                                        app.state.mode = Mode::Insert(Field::Tag);
                                    }, 
                                    Field::Tag => {
                                        app.state.buffer.set_tag(&input_val);
                                        app.state.mode = Mode::Insert(Field::Date);
                                    },
                                    Field::Date => {
                                        app.state.buffer.set_date(&input_val);
                                        app.state.mode = Mode::Insert(Field::Amount);
                                    },
                                    Field::Amount => {
                                        app.state.buffer.set_amount(&input_val);
                                        app.datastore.append_one(&app.state.buffer)?;
                                        app.state.buffer = Expense::empty();
                                    }
                                }
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
                Constraint::Percentage(30),
                Constraint::Percentage(35),
                Constraint::Percentage(35)
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
    let input_block = render_input_block(app, Field::Name);
    let input_block_inner_area = input_block.inner(input_stack_left[0]);
    frame.render_widget(input_block, input_stack_left[0]);
    let block_content = render_input(app, Field::Name);
    frame.render_widget(Block::default().title(block_content), input_block_inner_area);

    // Tag Input 
    let input_block = render_input_block(app, Field::Tag);
    let input_block_inner_area = input_block.inner(input_stack_left[1]);
    frame.render_widget(input_block, input_stack_left[1]);
    let block_content = render_input(app, Field::Tag);
    frame.render_widget(Block::default().title(block_content), input_block_inner_area);

    // Date Input 
    let input_block = render_input_block(app, Field::Date);
    let input_block_inner_area = input_block.inner(input_stack_right[0]);
    frame.render_widget(input_block, input_stack_right[0]);
    let block_content = render_input(app, Field::Date);
    frame.render_widget(Block::default().title(block_content), input_block_inner_area);

    // Amount Input
    let input_block = render_input_block(app, Field::Amount);
    let input_block_inner_area = input_block.inner(input_stack_right[1]);
    frame.render_widget(input_block, input_stack_right[1]);
    let block_content = render_input(app, Field::Amount);
    frame.render_widget(Block::default().title(block_content), input_block_inner_area);

    // Help Menu 
    frame.render_widget(instruction_block, chunk[0]);

    //Panel
    let titles = ["Overview", "Recent Records"].iter().cloned().map(Spans::from).collect();
    let panel_block = Block::default().borders(Borders::ALL);
    let tabs_block_inner_area = panel_block.inner(stack[1]);
    let tabs_block_inner_area = panel_block.inner(tabs_block_inner_area);
    let tabs = Tabs::new(titles)
        .block(panel_block)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider("|");
    frame.render_widget(tabs, stack[1]);
    frame.render_widget(render_table(app), tabs_block_inner_area);
}


// UI Component Render 
fn render_table(app: &Controller) -> Table {
    let data = app.datastore.list_all().unwrap();

    let header = Row::new(vec![
        Cell::from("Title"),
        Cell::from("Tag"),
        Cell::from("Date"),
        Cell::from("Amount")
    ])
        .style(Style::default().fg(Color::Yellow));

    let mut rows = Vec::new();
    for record in data {
        let li = Row::new(vec![
            Cell::from(record.name()),
            Cell::from(record.tag()),
            Cell::from(
                Utc.timestamp_nanos(record.date()).date().to_string()
            ),
            Cell::from(record.amount().to_string())
        ]);
        rows.push(li);
    };

    return Table::new(rows)// As any other widget, a Table can be wrapped in a Block.
    .block(Block::default())
    .widths(&[Constraint::Percentage(25), Constraint::Percentage(25), Constraint::Percentage(25), Constraint::Percentage(25)])
    .header(header)
    .column_spacing(1);
}

fn render_input<'a>(app: &Controller, field: Field) -> Span<'a> {
    if let Mode::Insert(f) = &app.state.mode {
        if field == *f {
            return Span::from(app.state.input.clone());
        }
        match field {
            Field::Name => {
                return Span::from(app.state.buffer.name());
            }, 
            Field::Tag => {
                return Span::from(app.state.buffer.tag());
            },
            Field::Date => {
                let date_timestamp = app.state.buffer.date();
                if date_timestamp == 0 {
                    return Span::from(String::new());
                }
                return Span::from(Utc.timestamp_nanos(date_timestamp).date().to_string());
            },
            Field::Amount => {
                return Span::from(app.state.buffer.amount().to_string());
            }
        }
    }
    return Span::from("");
}

fn render_input_block<'a>(app: &Controller, field: Field) -> Block<'a> {
    let name = match field { 
        Field::Name => "Title",
        Field::Tag => "Tag",
        Field::Amount => "Amount",
        Field::Date => "Date"
    };

    if let Mode::Insert(f) = &app.state.mode {
        if field == *f {
            return Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .title(Span::styled(name, Style::default().fg(Color::Yellow)));
        }
    }

    Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(name, Style::default().fg(Color::Yellow)))
}
