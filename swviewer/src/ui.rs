//! Create an immersive console UI experience.

use crate::symbolscanner::TraceVar;
use crossterm::event;
use std::io;
use std::sync::mpsc;
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, Row, SelectableList, Table, Text, Widget};
use tui::Terminal;

/// Command send from the UI to the processing thread.
pub enum UiThreadCommand {
    Stop,
    ConfigChannel { var: TraceVar, channel: usize },
}

/// Events to the UI
pub enum UiInput {
    Log(String),
}

struct UiState {
    quit: bool,
    index: usize,
    variables: Vec<TraceVar>,
    logs: Vec<String>,
    cmd_tx: mpsc::Sender<UiThreadCommand>,
    menu_items: Vec<String>,
    configured_channels: Vec<Option<TraceVar>>,
}

impl UiState {
    fn new(variables: Vec<TraceVar>, cmd_tx: mpsc::Sender<UiThreadCommand>) -> Self {
        let menu_items: Vec<String> = variables
            .iter()
            .map(|v| format!("{} @ 0x{:08X}", v.name, v.address))
            .collect();

        UiState {
            quit: false,
            index: 0,
            variables,
            cmd_tx,
            logs: vec![],
            menu_items,
            configured_channels: vec![None; 4],
        }
    }

    /// Handle terminal input event
    fn handle_event(&mut self, input_event: event::Event) {
        match input_event {
            event::Event::Key(key) => match key.code {
                event::KeyCode::Esc => self.quit(),
                event::KeyCode::Char(c) => match c {
                    '1' => self.select_variable(0),
                    '2' => self.select_variable(1),
                    '3' => self.select_variable(2),
                    '4' => self.select_variable(3),
                    'q' => self.quit(),
                    _ => {}
                },
                event::KeyCode::Enter => self.select_variable(1),
                event::KeyCode::Up => self.move_up(1),
                event::KeyCode::Down => self.move_down(1),
                event::KeyCode::PageUp => self.move_up(10),
                event::KeyCode::PageDown => self.move_down(10),
                event::KeyCode::Home => self.move_home(),
                event::KeyCode::End => self.move_end(),
                _ => {}
            },
            _ => {}
        }
    }

    fn move_up(&mut self, amount: usize) {
        if self.index > amount {
            self.index -= amount
        } else {
            self.index = 0;
        }
    }

    fn move_down(&mut self, amount: usize) {
        if !self.variables.is_empty() {
            if self.index + amount < self.variables.len() {
                self.index += amount
            } else {
                self.index = self.variables.len() - 1;
            }
        }
    }

    fn move_home(&mut self) {
        self.index = 0;
    }

    fn move_end(&mut self) {
        self.index = self.variables.len();
    }

    fn select_variable(&mut self, channel: usize) {
        let variable = self.variables[self.index].clone();
        if channel < 4 {
            self.configured_channels[channel] = Some(variable.clone());
            self.send_cmd(UiThreadCommand::ConfigChannel {
                channel,
                var: variable.clone(),
            });
        }
    }

    fn quit(&mut self) {
        self.send_cmd(UiThreadCommand::Stop);
        self.quit = true;
    }

    fn send_cmd(&self, cmd: UiThreadCommand) {
        if let Err(err) = self.cmd_tx.send(cmd) {
            // Oempf
        }
    }

    fn handle_background_event(&mut self, event: UiInput) {
        match event {
            UiInput::Log(txt) => {
                self.logs.push(txt);
            }
        }
    }

    fn render<B>(&self, terminal: &mut Terminal<B>) -> Result<(), io::Error>
    where
        B: Backend,
    {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.size());

            let chunks2 = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunks[0]);

            self.draw_variable_list(&mut f, chunks2[0]);
            self.draw_logs(&mut f, chunks2[1]);
            self.draw_channel_config(&mut f, chunks[1]);
        })?;

        Ok(())
    }

    /// Render the variable selection list
    fn draw_variable_list<B>(&self, f: &mut tui::Frame<B>, area: Rect)
    where
        B: Backend,
    {
        SelectableList::default()
            .block(
                Block::default()
                    .title("Select variable")
                    .borders(Borders::ALL),
            )
            .items(&self.menu_items)
            .select(Some(self.index))
            .style(Style::default().fg(Color::White).bg(Color::Cyan))
            .highlight_style(Style::default().modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
            .render(f, area);
    }

    /// Render state of channel configuration
    fn draw_channel_config<B>(&self, f: &mut tui::Frame<B>, area: Rect)
    where
        B: Backend,
    {
        let headers = ["Comparator", "Variable"].iter();

        let rows: Vec<Vec<String>> = self
            .configured_channels
            .iter()
            .enumerate()
            .map(|(i, v)| {
                vec![
                    format!("Channel {}", i),
                    match v {
                        None => "-".to_owned(),
                        Some(v) => v.name.clone(),
                    },
                ]
            })
            .collect();

        Table::new(headers, rows.iter().map(|r| Row::Data(r.iter())))
            .block(
                Block::default()
                    .title("Channel configuration")
                    .borders(Borders::ALL),
            )
            .column_spacing(1)
            .widths(&[
                Constraint::Length(10),
                Constraint::Length(50),
                Constraint::Length(10),
            ])
            .style(Style::default().fg(Color::Black).bg(Color::Yellow))
            .render(f, area);
    }

    fn draw_logs<B>(&self, f: &mut tui::Frame<B>, area: Rect)
    where
        B: Backend,
    {
        let items = self.logs.iter().map(|r| Text::raw(r));
        List::new(items)
            .block(Block::default().borders(Borders::ALL).title("logs"))
            .style(Style::default().fg(Color::Black).bg(Color::Gray))
            .render(f, area)
    }
}

pub fn run_tui(
    variables: Vec<TraceVar>,
    cmd_tx: mpsc::Sender<UiThreadCommand>,
    event_rx: mpsc::Receiver<UiInput>,
) -> Result<(), io::Error> {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    let mut ui_state = UiState::new(variables, cmd_tx);
    crossterm::terminal::enable_raw_mode().unwrap();

    use std::time::Duration;

    while !ui_state.quit {
        // Draw terminal:
        ui_state.render(&mut terminal)?;

        // Handle events:
        if crossterm::event::poll(Duration::from_millis(30)).unwrap() {
            let input_event = crossterm::event::read().unwrap();
            ui_state.handle_event(input_event);
        }

        if let Ok(event) = event_rx.try_recv() {
            ui_state.handle_background_event(event);
        }
    }

    crossterm::terminal::disable_raw_mode().unwrap();
    Ok(())
}
