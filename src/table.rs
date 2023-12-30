use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

use crate::posts::Post;
struct App {
    state: TableState,
    items: Vec<Post>,
}

pub enum Mode {
    Select, 
    Delete,
    Default,
}

impl App {
    fn new(items: Vec<Post>) -> App {
        App {
            state: TableState::default(),
            items
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub fn draw_table(post: &Vec<Post>) -> Result<(TableState,Mode), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new(post.to_vec());
    let res = run_app(&mut terminal, &mut app)?;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok((app.state,res))
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<Mode> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(Mode::Default),
                    KeyCode::Down | KeyCode::Char('j') => app.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous(),
                    KeyCode::Enter => return Ok(Mode::Select),
                    KeyCode::Char('d') => return Ok(Mode::Delete),
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(100)])
        .split(f.size());

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let rows = app.items.iter().map(|item| {
        let vec_items = item.to_array(); 
        let height = vec_items 
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = vec_items.iter().map(|c| Cell::from(c.clone()));

        Row::new(cells).height(height as u16).bottom_margin(1)
    });
    let t = Table::new(
        rows,
        [
            Constraint::Percentage(15),
            Constraint::Percentage(20),
            Constraint::Percentage(10),
            Constraint::Percentage(40),
            Constraint::Percentage(10),
        ],
    )
    .block(Block::default().borders(Borders::ALL).title("Posts"))
    .highlight_style(selected_style)
    .highlight_symbol(">> ");
    f.render_stateful_widget(t, rects[0], &mut app.state);
}
