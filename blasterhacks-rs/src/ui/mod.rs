mod app;

use crate::types::data::Data;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Modifier},
    symbols,
    text::Span,
    widgets::{
        Axis, Block, Borders, Cell, Chart, Dataset, GraphType, List, ListItem, Paragraph, Row,
        Table, Wrap,
    },
    Frame, Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use app::{App, Dir};

fn render_assignments<B: Backend>(f: &mut Frame<B>, app: &mut App, layout_chunk: Rect) {
    let bold = Style::default().fg(Color::White).add_modifier(Modifier::BOLD);
    let header_cells = ["Course", "Name", "Due Date"].iter().map(|h| Cell::from(*h));
    let header = Row::new(header_cells)
        .style(bold)
        .height(1);
    let rows = app.data.assignments.iter().map(|a| {
        let date = if let Some(date) = a.date {
            date.format("%A %d, %H:%M").to_string()
        } else {
            "(No due date)".to_string()
        };
        let cells = vec![
            format!("{}", a.course),
            format!("{}", a.name),
            format!("{}", date),
        ];
        Row::new(cells)
        
    });
    let selected_style = Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD);
    let table = Table::new(rows)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Upcoming Assignments")
        )
        .highlight_style(selected_style)
        .widths(&[
            Constraint::Ratio(1, 10),
            Constraint::Ratio(6, 10),
            Constraint::Ratio(3, 10),
        ]);
    f.render_stateful_widget(table, layout_chunk, &mut app.assignments_state);
}

fn render_grades<B: Backend>(f: &mut Frame<B>, app: &mut App, layout_chunk: Rect) {
    let bold = Style::default().fg(Color::White).add_modifier(Modifier::BOLD);
    let header_cells = ["Course", "Grade"].iter().map(|h| Cell::from(*h));
    let header = Row::new(header_cells)
        .style(bold)
        .height(1);
    let rows = app.data.grades.iter().map(|g| {
        let cells = vec![
            format!("{}", g.course),
            format!("{}", g.grade),
        ];
        Row::new(cells)
        
    });
    let table = Table::new(rows)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Grades")
        )
        .widths(&[
            Constraint::Ratio(1, 2),
            Constraint::Ratio(1, 2),
        ]);
    f.render_stateful_widget(table, layout_chunk, &mut app.assignments_state);
}

fn render_welcome<B: Backend>(f: &mut Frame<B>, app: &mut App, layout_chunk: Rect) {
    let welcome = Paragraph::new(format!("\nToday is {}, there are {} upcoming assignments", chrono::Local::now().format("%A %d %B"), app.data.assignments.len()))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Welcome to CanvasTUI")
        )
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(welcome, layout_chunk);
}

fn render_plan<B: Backend>(f: &mut Frame<B>, app: &mut App, layout_chunk: Rect) {
    let plan = Paragraph::new(app.data.plan.clone())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Study Plan")
        )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(plan, layout_chunk);
}

fn render_summary<B: Backend>(f: &mut Frame<B>, app: &mut App, layout_chunk: Rect) {
    let summary = if let Some(i) = app.assignments_state.selected() {
        let assignment = &app.data.assignments[i];
        let s = format!("Course: {}\nName: {}\nSummary: {}\n", assignment.course, assignment.name, assignment.summary.as_ref().unwrap_or(&"No summary".to_string()));
        s
    } else {
        "No assignment selected".to_string()
    };
    let info = Paragraph::new(summary)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Assignment Summary")
        )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(info, layout_chunk);
}

fn render_links<B: Backend>(f: &mut Frame<B>, app: &mut App, layout_chunk: Rect) {
    if let Some(i) = app.assignments_state.selected() {
        let links = app.data.assignments[i].relevant_links.clone().into_iter().map(|link| {
            ListItem::new(link)
        }).collect::<Vec<_>>();
        if links.is_empty() {
            let empty = Paragraph::new("No links available")
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Links")
                )
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true });
            f.render_widget(empty, layout_chunk);
            return;
        }
        let table = List::new(links)
            .block(
                Block::default()
                .borders(Borders::ALL)
                .title("Links")
            );
        f.render_stateful_widget(table, layout_chunk, &mut app.links_state);
        return;
    }
    let empty = Paragraph::new("No assignment selected")
        .block(
            Block::default()
            .borders(Borders::ALL)
            .title("Links")
        )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(empty, layout_chunk);
}

fn render_default<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(2, 3), Constraint::Ratio(1, 3)].as_ref())
        .split(f.size());

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 10), Constraint::Ratio(6, 10), Constraint::Ratio(3, 10)].as_ref())
        .split(chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(3, 10), Constraint::Ratio(7, 10)].as_ref())
        .split(chunks[1]);

    let bottom_left_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
        .split(left_chunks[2]);

    render_welcome(f, app, left_chunks[0]);
    render_assignments(f, app, left_chunks[1]);
    render_summary(f, app, bottom_left_chunks[0]);
    render_links(f, app, bottom_left_chunks[1]);
    render_grades(f, app, right_chunks[0]);
    render_plan(f, app, right_chunks[1]);
}


pub fn run(data: Data) -> Result<(), Box<dyn Error>>{
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new(data, Duration::from_millis(1000));

    // Main loop and tick logic
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| render_default(f, &mut app))?;

        // Non-blocking key detection
        let timeout = app
            .tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::from_secs(0));
        if event::poll(timeout)? {
            if handle_input(&mut app)? {
                break;
            }
        }
        if last_tick.elapsed() >= app.tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }


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

fn handle_input(app: &mut App) -> Result<bool, Box<dyn Error>> {
    if let Event::Key(key) = event::read()? {
        match key.modifiers {
            KeyModifiers::NONE => match key.code {
                KeyCode::Char('j') => app.mv(Dir::Down),
                KeyCode::Char('k') => app.mv(Dir::Up),
                KeyCode::Char('q') => return Ok(true),
                _ => (),
            },
            KeyModifiers::CONTROL => match key.code {
                _ => (),
            },
            _ => (),
        }
    }
    Ok(false)
}

