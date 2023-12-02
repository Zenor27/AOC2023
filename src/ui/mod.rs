use std::io;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

use crate::aoc2023::day_to_solve_functions::DayToSolveFunctions;
use crate::aoc2023::result::AdventOfCodeResult;

struct DaysList<'a> {
    state: ListState,
    day_to_solver_functions: &'a DayToSolveFunctions,
}

impl DaysList<'_> {
    fn with_items(day_to_solver_functions: &DayToSolveFunctions) -> DaysList {
        DaysList {
            state: ListState::default().with_selected(Some(0)),
            day_to_solver_functions,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.day_to_solver_functions.keys().len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.day_to_solver_functions.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn run_solvers(&mut self) -> (&'static str, Vec<(usize, AdventOfCodeResult)>) {
        let day_index = self.state.selected().unwrap();
        let day_name = self
            .day_to_solver_functions
            .keys()
            .rev()
            .nth(day_index)
            .unwrap();
        let solvers = self.day_to_solver_functions.get(day_name).unwrap();
        return (
            day_name,
            solvers
                .iter()
                .enumerate()
                .map(|(index, solve_function)| {
                    return (index, solve_function());
                })
                .collect::<Vec<(usize, AdventOfCodeResult)>>(),
        );
    }
}

struct App<'a> {
    days_list: DaysList<'a>,
}

impl App<'_> {
    fn new(day_to_solve_functions: &DayToSolveFunctions) -> App {
        App {
            days_list: DaysList::with_items(day_to_solve_functions),
        }
    }
}

pub fn run_cli(day_to_solve_functions: &DayToSolveFunctions) {
    enable_raw_mode().expect("Failed to enable raw mode");

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).expect("Failed to enter alternate screen");

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Failed to initialize terminal");

    let app = App::new(day_to_solve_functions);

    let _ = run_app(&mut terminal, app);
    disable_raw_mode().expect("Failed to disable raw mode");
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)
        .expect("Failed to leave alternate screen");
    terminal.show_cursor().expect("Failed to show cursor");
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| select_day_ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Enter => {
                        let (day, results) = app.days_list.run_solvers();
                        terminal.clear()?;
                        terminal.draw(|f| results_ui(f, results, day))?;
                        event::read()?;
                        return Ok(());
                    }
                    KeyCode::Down => app.days_list.next(),
                    KeyCode::Up => app.days_list.previous(),
                    _ => {}
                }
            }
        }
    }
}

fn select_day_ui(f: &mut Frame, app: &mut App) {
    let chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .split(f.size())[0];

    let items: Vec<ListItem> = app
        .days_list
        .day_to_solver_functions
        .keys()
        .rev()
        .map(|day_name| {
            let mut lines = Vec::new();
            lines.push(format!("{}", day_name).bold().fg(Color::Black).into());
            return ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White));
        })
        .collect();

    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Days"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(items, chunk, &mut app.days_list.state);
}

fn results_ui(f: &mut Frame, results: Vec<(usize, AdventOfCodeResult)>, day: &str) {
    let chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .split(f.size())[0];

    let items: Vec<ListItem> = results
        .iter()
        .map(|(part_index, result)| {
            let mut lines = Vec::new();
            let result_string = match result {
                Ok(result) => format!("{}", result),
                Err(error) => format!("{}", error),
            };
            let result_fg = match result {
                Ok(_) => Color::Green,
                Err(_) => Color::Red,
            };
            lines.push(
                format!("Part {}: {}", part_index + 1, result_string)
                    .bold()
                    .fg(result_fg)
                    .into(),
            );
            return ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White));
        })
        .collect();

    let items = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Results for {}", day)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_widget(items, chunk);
}
