use color_eyre::{Result, eyre::Ok};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, BorderType, List, ListItem, Paragraph, Widget},
};
use std::io;

#[derive(Debug, Default)]
struct ToDoItem {
    is_done: bool,
    description: String,
}

#[derive(Debug, Default)]
struct AppState {
    items: Vec<ToDoItem>,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut state = AppState::default();

    state.items.push(ToDoItem {
        is_done: false,
        description: String::from("Hello"),
    });

    state.items.push(ToDoItem {
        is_done: false,
        description: String::from("?"),
    });

    state.items.push(ToDoItem {
        is_done: true,
        description: String::from("??"),
    });

    ratatui::run(|terminal| run(terminal, &mut state))
}

pub fn run(terminal: &mut DefaultTerminal, app_state: &AppState) -> Result<()> {
    loop {
        // Render
        terminal.draw(|frame| render(frame, app_state))?;
        //Input
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    break;
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, app_state: &AppState) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());

    let [inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(border_area);

    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(Color::Blue)
        .title("Tasks")
        .render(frame.area(), frame.buffer_mut());
    List::new(
        app_state
            .items
            .iter()
            .map(|x| ListItem::from(x.description.clone())),
    )
    .render(inner_area, frame.buffer_mut());
}
