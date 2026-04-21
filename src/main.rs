use color_eyre::{Result, eyre::Ok};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint, Layout},
    widgets::{Block, BorderType, Clear, Paragraph},
};

mod tasks;

use tasks::render_tasks;
use tasks::{TaskState, ToDoItem};

enum FormAction {
    None,
    Submit,
    Escape,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut state = TaskState::default();

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

fn run(terminal: &mut DefaultTerminal, app_state: &mut TaskState) -> Result<()> {
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
                        app_state.task_input_val.clear();
                    }
                    FormAction::Submit => {
                        app_state.items.push(ToDoItem {
                            description: app_state.task_input_val.clone(),
                            is_done: false,
                        });
                        app_state.is_adding = false;
                        app_state.task_input_val.clear();
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

fn handle_add_key_input(key: KeyEvent, app_state: &mut TaskState) -> FormAction {
    match key.code {
        KeyCode::Esc => {
            return FormAction::Escape;
        }
        KeyCode::Enter => {
            return FormAction::Submit;
        }
        KeyCode::Char(c) => {
            app_state.task_input_val.push(c);
        }
        KeyCode::Backspace => {
            app_state.task_input_val.pop();
        }
        _ => {}
    }
    FormAction::None
}

fn handle_key_input(key: KeyEvent, app_state: &mut TaskState) -> bool {
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

fn render(frame: &mut Frame, app_state: &mut TaskState) {
    let border_area = Layout::horizontal(vec![
        Constraint::Percentage(20),
        Constraint::Percentage(30),
        Constraint::Percentage(50),
    ])
    .margin(1)
    .split(frame.area());

    render_tasks(frame, app_state, border_area[1]);
}

pub fn render_popup(frame: &mut Frame, app_state: &mut TaskState, title: String) {
    let popup_block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title(title)
        .title_alignment(Alignment::Center);
    let centered_area = frame
        .area()
        .centered(Constraint::Percentage(60), Constraint::Percentage(20));
    frame.render_widget(Clear, centered_area);
    let paragraph = Paragraph::new(app_state.task_input_val.as_str()).block(popup_block);
    frame.render_widget(paragraph, centered_area);
}
