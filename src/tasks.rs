use ratatui::widgets::ListState;

use crate::render_popup;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::ToSpan,
    widgets::{Block, BorderType, List, ListItem, Widget},
};

#[derive(Debug, Default)]
pub struct ToDoItem {
    pub is_done: bool,
    pub description: String,
}

#[derive(Debug, Default)]
pub struct TaskState {
    pub items: Vec<ToDoItem>,
    pub list_state: ListState,
    pub is_adding: bool,
    pub task_input_val: String,
}

pub fn render_tasks(frame: &mut Frame, app_state: &mut TaskState, area: Rect) {
    let [inner_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(area);

    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(Color::Blue)
        .title("Tasks")
        .render(area, frame.buffer_mut());
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
        render_popup(frame, app_state, String::from("Add a task"));
    }
}
