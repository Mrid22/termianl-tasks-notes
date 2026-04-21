use color_eyre::{Result, eyre::Ok};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Text, ToSpan},
    widgets::{Block, BorderType, Clear, List, ListItem, ListState, Paragraph, Widget},
};
use std::{io, iter};

enum FormAction {
    None,
    Submit,
    Escape,
}

#[derive(Debug, Default)]
struct ToDoItem {
    is_done: bool,
    description: String,
}

#[derive(Debug, Default)]
struct AppState {
    items: Vec<ToDoItem>,
    list_state: ListState,
    is_adding: bool,
    input_val: String,
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

fn run(terminal: &mut DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    loop {
        // Render
        terminal.draw(|frame| render(frame, app_state))?;
        //Input
        if let Event::Key(key) = event::read()? {
            if app_state.is_adding {
                match handle_add_key_input(key, app_state) {
                    FormAction::None => {}
                    FormAction::Escape => {
                        app_state.is_adding = false;
                        app_state.input_val.clear();
                    }
                    FormAction::Submit => {
                        app_state.items.push(ToDoItem {
                            description: app_state.input_val.clone(),
                            is_done: false,
                        });
                        app_state.is_adding = false;
                        app_state.input_val.clear();
                    }
                }
            } else {
                if handle_key_input(key, app_state) {
                    break;
                }
            }
        }
    }
    Ok(())
}

fn handle_add_key_input(key: KeyEvent, app_state: &mut AppState) -> FormAction {
    match key.code {
        KeyCode::Esc => {
            return FormAction::Escape;
        }
        KeyCode::Enter => {
            return FormAction::Submit;
        }
        KeyCode::Char(c) => {
            app_state.input_val.push(c);
        }
        KeyCode::Backspace => {
            app_state.input_val.pop();
        }
        _ => {}
    }
    FormAction::None
}

fn handle_key_input(key: KeyEvent, app_state: &mut AppState) -> bool {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            return true;
        }
        KeyCode::Enter | KeyCode::Char(' ') => {
            if let Some(index) = app_state.list_state.selected() {
                if let Some(item) = app_state.items.get_mut(index) {
                    item.is_done = !item.is_done;
                };
            }
        }
        KeyCode::Char(char) => match char {
            'j' => {
                app_state.list_state.select_next();
            }
            'k' => {
                app_state.list_state.select_previous();
            }
            'D' => {
                if let Some(index) = app_state.list_state.selected() {
                    app_state.items.remove(index);
                }
            }
            'A' => {
                app_state.is_adding = true;
            }
            _ => {}
        },
        _ => {}
    }
    false
}

fn render(frame: &mut Frame, app_state: &mut AppState) {
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
    let list = List::new(app_state.items.iter().map(|task| {
        let value = if task.is_done {
            task.description.to_span().crossed_out().dim()
        } else {
            task.description.to_span()
        };
        ListItem::from(task.description.as_str());
        ListItem::from(value)
    }))
    .highlight_symbol("> ")
    .highlight_style(Style::default().fg(Color::Yellow));

    frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);

    if app_state.is_adding {
        let popup_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title("Add a task")
            .title_alignment(Alignment::Center);
        let centered_area = frame
            .area()
            .centered(Constraint::Percentage(60), Constraint::Percentage(20));
        frame.render_widget(Clear, centered_area);
        let paragraph = Paragraph::new(app_state.input_val.as_str()).block(popup_block);
        frame.render_widget(paragraph, centered_area);
    }
}
