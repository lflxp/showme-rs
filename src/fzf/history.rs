use crate::fzf::App;
use std::{
    process::{self, Command}
};
use tui::{
    widgets::{ListItem, List,Block,Borders},
    style::{Modifier,Color,Style},
    backend::{Backend},
    text::{Span,Spans},
    layout::{Rect},
    Frame
};

pub fn draw_command<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    app.history_search.items.clear();
    for x in &app.history.items {
        if x.contains(&app.input) {
            app.history_search.items.push(x.to_string())
        }
    }

    let messages: Vec<ListItem> = app
        .history_search
        .items
        .iter()
        .map(|x| {
            ListItem::new(vec![Spans::from(Span::raw(x))]) 
        })
        .collect();
    
    // Create a List from all list items and highlight the currently selected one
    let items = List::new(messages)
        .block(Block::default().borders(Borders::ALL).title("历史命令"))
        .highlight_style(
            Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    // We can now render the item list
    f.render_stateful_widget(items, area, &mut app.history_search.state);
}