//! Create some immersive UI experience.

use crate::symbolscanner::TraceVar;
use std::io;
use tui::backend::CrosstermBackend;

pub fn run_tui<'t>(variables: &'t Vec<TraceVar>) -> Result<Option<&'t TraceVar>, io::Error> {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    use crossterm::event;
    use tui::layout::{Constraint, Layout};
    use tui::style::{Color, Modifier, Style};
    use tui::widgets::{Block, Borders, Row, SelectableList, Table, Widget};
    use tui::Terminal;

    // let inp2 = inp.read_sync();

    crossterm::terminal::enable_raw_mode().unwrap();

    let menu_items: Vec<String> = variables
        .iter()
        .map(|v| format!("{} @ 0x{:08X}", v.name, v.address))
        .collect();
    let mut index = 0;

    let selection = loop {
        // Draw terminal:
        terminal.draw(|mut f| {
            let size = f.size();

            // Block::default()
            //     .title("Block")
            //     .borders(Borders::ALL)
            //     .render(&mut f, size);
            let selected_style = Style::default().fg(Color::Yellow).modifier(Modifier::BOLD);
            let normal_style = Style::default().fg(Color::White);

            SelectableList::default()
                .block(
                    Block::default()
                        .title("Select variable")
                        .borders(Borders::ALL),
                )
                .items(&menu_items)
                .select(Some(index))
                .style(Style::default().fg(Color::White).bg(Color::Blue))
                .highlight_style(Style::default().modifier(Modifier::ITALIC))
                .highlight_symbol(">>")
                .render(&mut f, size);
        })?;

        // Handle events:
        let input_event = crossterm::event::read().unwrap();
        // println!("Event: {:?}", event);

        match input_event {
            event::Event::Key(key) => match key.code {
                event::KeyCode::Char('q') | event::KeyCode::Esc => {
                    break None;
                }
                event::KeyCode::Char('c') => {
                    if key.modifiers.contains(event::KeyModifiers::CONTROL) {
                        break None;
                    }
                }
                event::KeyCode::Enter => {
                    break Some(&variables[index]);
                }
                event::KeyCode::Up => {
                    if index > 0 {
                        index -= 1
                    }
                }
                event::KeyCode::Down => {
                    if index < variables.len() - 1 {
                        index += 1
                    }
                }
                _ => {}
            },
            _ => {}
        }
    };

    crossterm::terminal::disable_raw_mode().unwrap();

    Ok(selection)
}
