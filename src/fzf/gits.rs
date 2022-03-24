use crate::fzf::App;
use std::{
    process::Command};
use tui::{
    backend::{Backend},
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Clear},
    Frame,
};

// ==================DRAW GIT COMMIT====================================
pub fn draw_commit<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let size = f.size();
    let constraints = if app.show_detail { 
        vec![
            Constraint::Percentage(40),
            Constraint::Percentage(60)
        ]
    } else {
        vec![Constraint::Percentage(100)]
    };

    let chunks = Layout::default()
        .constraints(constraints.as_ref())
        .direction(Direction::Horizontal)
        .split(area);

    app.gits_search.items.clear();
    // app.current = 0;
    // for x in &app.items.items {
    //     if x.contains(&app.input) {
    //         app.search.push(x.to_string());
    //     }
    // }
    for x in &app.gits.items {
        if x.contains(&app.input) {
            app.gits_search.items.push(x.to_string())
        }
    }
    let items: Vec<ListItem> = app
        .gits_search
        .items
        .iter()
        .map(|i| {
            ListItem::new(vec![Spans::from(Span::raw(i))])
        })
        .collect();

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Git Commit [Detail(F1)]"))
        .highlight_style(
            Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    // We can now render the item list
    f.render_stateful_widget(items, chunks[0], &mut app.gits_search.state);
    if app.show_detail {
        ui_text_git(f,app,chunks[1]);
    }

    // popup 显示git reset提示
    if app.show_popup {
        let filename = app.gits_search.items.get(app.current).unwrap();
        let paragraph = Paragraph::new(String::from(format!("git reset --hard {}",filename)))
            .style(Style::default().bg(Color::White).fg(Color::Black))
            .block(Block::default().title("是/否回退？").borders(Borders::ALL))
            .alignment(Alignment::Center);
        let area = centered_rect(60,20,size);
        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(paragraph, area);
    }
}

fn ui_text_git<B: Backend>(f: &mut Frame<B>, app: &mut App<'_>, area: Rect) {
    // Words made "loooong" to demonstrate line breaking.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                // Constraint::Percentage(25),
                // Constraint::Percentage(25),
                // Constraint::Percentage(25),
                // Constraint::Percentage(25),
                Constraint::Percentage(100),
            ]
            .as_ref(),
        )
        .split(area);

    app.gits_detail.items.clear();
    app.currentdetail = 0;
    let mut filename = &String::from("None");
    // let mut data: Vec<ListItem> = Vec::new();
    if app.gits_search.items.len() > 0 {
        filename = app.gits_search.items.get(app.current).unwrap();
        let output = Command::new("sh").arg("-c").arg(format!("git show {}",filename.split(' ').collect::<Vec<&str>>()[0])).output().expect("命令执行异常错误提示");
        let ls_la_list = String::from_utf8(output.stdout); 
        match ls_la_list {
            Ok(info) => {
                for x in info.lines() {
                    app.gits_detail.items.push(x.to_string());
                }
            },
            Err(e) => {
                app.gits_detail.items.push(format!("{}",e))
            }
        };
    } else {
        app.gits_detail.items.push(String::from("None"))
    }

    let items: Vec<ListItem> = app
        .gits_detail.items
        .iter()
        .map(|i| {
            ListItem::new(vec![Spans::from(Span::raw(i))])
        })
        .collect();

    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(format!("{} (Home/End)",filename)))
        .highlight_style(
            Style::default()
                .fg(Color::Green)
                .bg(Color::Gray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");
    f.render_stateful_widget(items, chunks[0],&mut app.gits_detail.state);
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}