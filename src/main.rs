// INSPIRATION: https://github.com/fdehau/tui-rs/blob/master/examples/user_input.rs

use std::io;

use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, List, ListItem, Row, Table, Tabs},
    Frame, Terminal,
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
use rbudget::record::*;
use rbudget::{data::*, syntax::parse_date};

struct Controller {
    datastore: DataStore,
    state: State,
}

#[derive(PartialEq)]
enum Field {
    Name,
    Tag,
    Date,
    Amount,
}

enum Mode {
    Normal,
    Insert(Field),
}

struct State {
    input: String,
    buffer: Expense,
    mode: Mode,
    msg: String,
}

impl Controller {
    fn new() -> Result<Controller, Error> {
        Ok(Controller {
            datastore: DataStore::new()?,
            state: State {
                input: String::new(),
                buffer: Expense::empty(),
                mode: Mode::Normal,
                msg: String::new(),
            },
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

// +++++++++++++++++++ EVENTS ++++++++++++++++++++ //
fn run<B: Backend>(terminal: &mut Terminal<B>, mut app: Controller) -> Result<(), Error> {
    loop {
        terminal.draw(|f| render(f, &app))?;

        let event = crossterm::event::read()?;
        match &app.state.mode {
            Mode::Normal => match event {
                Event::Key(key) => match key.code {
                    KeyCode::Char(ch) => {
                        app.exec(ch);
                    }
                    KeyCode::Esc => {
                        return Ok(());
                    }
                    _ => {}
                },
                _ => {}
            },
            Mode::Insert(field) => match event {
                Event::Key(key) => match key.code {
                    KeyCode::Backspace => {
                        app.state.input.pop();
                    }
                    KeyCode::Char(ch) => {
                        app.state.input.push(ch);
                    }
                    KeyCode::Tab | KeyCode::Enter => {
                        let input_val = app.state.input;
                        match field {
                            Field::Name => {
                                app.state.buffer.set_name(&input_val);
                                app.state.mode = Mode::Insert(Field::Date);
                            }
                            Field::Tag => {
                                app.state.buffer.set_tag(&input_val);
                                app.state.mode = Mode::Insert(Field::Amount);
                            }
                            Field::Date => {
                                let nd = parse_date(&input_val);
                                match nd {
                                    Err(_e) => {
                                        app.state.msg =
                                            String::from("ERROR: cannot read date input.");
                                    }
                                    Ok(date) => {
                                        app.state.buffer.set_date(&date.to_string());
                                        app.state.mode = Mode::Insert(Field::Tag);
                                        app.state.msg.clear();
                                    }
                                }
                            }
                            Field::Amount => {
                                let res = app.state.buffer.set_amount(&input_val);
                                if let Err(e) = res {
                                    app.state.msg = String::from("ERROR: cannot parse amount");
                                } else {
                                    app.state.mode = Mode::Insert(Field::Name);
                                }
                            }
                        }
                        // Register record when user hits 'Enter' key.
                        if key.code == KeyCode::Enter {
                            if app.state.buffer.name() == "" {
                                app.state.msg = String::from("ERROR: record name is empty.");
                                app.state.mode = Mode::Insert(Field::Name);
                            } else if app.state.buffer.date() == "" {
                                app.state.msg = String::from("ERROR: record does not have a date.");
                                app.state.mode = Mode::Insert(Field::Date);
                            } else if app.state.buffer.amount() == 0 {
                                app.state.msg =
                                    String::from("ERROR: record does not have an amount.");
                                app.state.mode = Mode::Insert(Field::Amount);
                            } else {
                                app.datastore.append_one(&app.state.buffer)?;
                                app.state.buffer = Expense::empty();
                            }
                        }
                        app.state.input = String::new();
                    }
                    KeyCode::Esc => {
                        app.state.mode = Mode::Normal;
                    }
                    _ => {}
                },
                _ => {}
            },
        }
    }
}

// +++++++++++++++++++ LAYOUT ++++++++++++++++++++ //
fn render<B: Backend>(frame: &mut Frame<B>, app: &Controller) {
    // Layout: divide page into top and bottom
    let stack = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(24),
                Constraint::Percentage(64),
                Constraint::Percentage(12),
            ]
            .as_ref(),
        )
        .split(frame.size());

    let chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(30),
                Constraint::Percentage(40),
            ]
            .as_ref(),
        )
        .split(stack[0]);

    let input_columns= Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]);

    // +++++++++++++++++++ ASSEMBLE ++++++++++++++++++++ //
    // Input fields and blocks
    let input_stack_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref());
    let input_container = Block::default().title("Add new record:").borders(Borders::ALL).border_style(Style::default().fg(Color::LightGreen));
    let input_chunks = input_columns.split(input_container.inner(chunk[2]));
    frame.render_widget(input_container, chunk[2]);
    let input_stack_left = input_stack_layout.split(input_chunks[0]);
    let input_stack_right = input_stack_layout.split(input_chunks[1]);

    // Name Input
    let input_block = render_input_block(app, Field::Name);
    let input_block_inner_area = input_block.inner(input_stack_left[0]);
    frame.render_widget(input_block, input_stack_left[0]);
    let block_content = render_input(app, Field::Name);
    frame.render_widget(
        Block::default().title(block_content),
        input_block_inner_area,
    );

    // Tag Input
    let input_block = render_input_block(app, Field::Tag);
    let input_block_inner_area = input_block.inner(input_stack_left[1]);
    frame.render_widget(input_block, input_stack_left[1]);
    let block_content = render_input(app, Field::Tag);
    frame.render_widget(
        Block::default().title(block_content),
        input_block_inner_area,
    );

    // Date Input
    let input_block = render_input_block(app, Field::Date);
    let input_block_inner_area = input_block.inner(input_stack_right[0]);
    frame.render_widget(input_block, input_stack_right[0]);
    let block_content = render_input(app, Field::Date);
    frame.render_widget(
        Block::default().title(block_content),
        input_block_inner_area,
    );

    // Amount Input
    let input_block = render_input_block(app, Field::Amount);
    let input_block_inner_area = input_block.inner(input_stack_right[1]);
    frame.render_widget(input_block, input_stack_right[1]);
    let block_content = render_input(app, Field::Amount);
    frame.render_widget(
        Block::default().title(block_content),
        input_block_inner_area,
    );

    // Help Menu
    frame.render_widget(render_menu(), chunk[0]);

    // Message
    let msg_block = Block::default().title("Log").borders(Borders::ALL).border_style(Style::default().fg(Color::LightGreen));
    frame.render_widget(render_msg(app), msg_block.inner(stack[2]));
    frame.render_widget(msg_block, stack[2]);

    // Tabs
    let titles = ["Overview", "Recent Records"]
        .iter()
        .cloned()
        .map(Spans::from)
        .collect();
    let panel_block = Block::default().borders(Borders::ALL);
    let tabs_block_inner_area = panel_block.inner(stack[1]);
    let tabs_block_inner_area = panel_block.inner(tabs_block_inner_area);
    let tabs = Tabs::new(titles)
        .block(panel_block)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::LightBlue))
        .divider("|");
    frame.render_widget(tabs, stack[1]);
    frame.render_widget(render_table(app), tabs_block_inner_area);
}

// +++++++++++++++++++ COMPONENTS ++++++++++++++++++++ //
fn render_menu() -> List<'static> {
    let items = [
        ListItem::new("Press i to edit"),
        ListItem::new("Press Esc to quit."),
        ListItem::new("Press Tab to navigate."),
        ListItem::new("Press ? for a complete help."),
    ];
    List::new(items).block(Block::default().title("Help").borders(Borders::ALL).border_style(Style::default().fg(Color::LightGreen)))
}

fn render_msg(app: &Controller) -> Block {
    let msg_txt = Span::from(app.state.msg.clone());
    Block::default().title(msg_txt)
}

fn render_table<'a>(app: &'a Controller) -> Table<'a> {
    let data = app.datastore.list_all().unwrap();

    let header = Row::new(vec![
        Cell::from("Title"),
        Cell::from("Tag"),
        Cell::from("Date"),
        Cell::from("Amount"),
    ])
    .style(Style::default().fg(Color::LightBlue));

    let mut rows = Vec::new();
    for record in data {
        let li = Row::new(vec![
            Cell::from(record.name()),
            Cell::from(record.tag()),
            Cell::from(record.date()),
            Cell::from(record.amount().to_string()),
        ]);
        rows.push(li);
    }

    return Table::new(rows)
        .block(Block::default())
        .widths(&[
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .header(header)
        .column_spacing(1);
}

fn render_input<'a>(app: &'a Controller, field: Field) -> Span<'a> {
    if let Mode::Insert(f) = &app.state.mode {
        if field == *f {
            return Span::from(app.state.input.clone());
        }
        match field {
            Field::Name => {
                return Span::from(app.state.buffer.name());
            }
            Field::Tag => {
                return Span::from(app.state.buffer.tag());
            }
            Field::Date => {
                let date_str = app.state.buffer.date();
                if date_str == "" {
                    return Span::from(String::new());
                }
                return Span::from(date_str);
            }
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
        Field::Date => "Date",
    };

    if let Mode::Insert(f) = &app.state.mode {
        if field == *f {
            return Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::LightBlue))
                .title(Span::styled(name, Style::default().fg(Color::LightBlue)));
        }
    }

    Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(name, Style::default().fg(Color::LightBlue)))
}
