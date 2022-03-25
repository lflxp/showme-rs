use crate::fzf::{App, centered_rect, execShell};
use std::{
    process::{self, Command}
};
use tui::{
    widgets::{ListItem, List,Block,Borders,Paragraph, Clear},
    style::{Modifier,Color,Style},
    backend::{Backend},
    text::{Span,Spans},
    layout::{Rect,Alignment, Layout, Direction, Constraint},
    Frame
};

// 设计：1、kubectl get po -A 2、kubectl edit po -n a b 3、kubectl get po -n a b -o yaml

pub fn draw_k8s<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let size = f.size();
    // 准备数据
    app.kind_search.items.clear();
    for x in &app.kind_data.items {
        if x.contains(&app.input) {
            app.kind_search.items.push(x.to_string());
        }
    }

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

    let messages: Vec<ListItem> = app
        .kind_search
        .items
        .iter()
        .map(|x| {
            ListItem::new(vec![Spans::from(Span::raw(x))]) 
        })
        .collect();
    
    // Create a List from all list items and highlight the currently selected one
    let items = List::new(messages)
        .block(Block::default().borders(Borders::ALL).title(app.kind.to_string()))
        .highlight_style(
            Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    // We can now render the item list
    f.render_stateful_widget(items, chunks[0], &mut app.kind_search.state);
    if app.show_detail {
        ui_text_k8s(f,app,chunks[1]);
    }

    if app.kind_input {
        // let filename = app.kind_search.items.get(app.current).unwrap();
        let paragraph = Paragraph::new(app.kind.as_ref())
            .style(Style::default().bg(Color::White).fg(Color::Black))
            .block(Block::default().title("修改查询类型 F6(on/off)").borders(Borders::ALL))
            .alignment(Alignment::Center);
        let area2 = centered_rect(60,20,size);
        f.render_widget(Clear, area2); // this clears out the background
        f.render_widget(paragraph, area2);
    }
}

// fn ui_change_kind<B: Backend>(f: &mut Frame<B>, app: &mut App<'_>, area: Rect) {
    
// }

fn ui_text_k8s<B: Backend>(f: &mut Frame<B>, app: &mut App<'_>, area: Rect) {
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

    // app.kind_detail.items.clear();
    // app.kinddetail = 0;
    let mut filename = &String::from("None");
    // let mut data: Vec<ListItem> = Vec::new();
    if app.kind_detail.items.len() == 0 {
        filename = app.kind_search.items.get(app.current).unwrap();
        let tmp = filename.split(' ').collect::<Vec<&str>>();
        let output = Command::new("sh").arg("-c").arg(format!("kubectl get {} -n {} {} -o yaml",app.kind,tmp[0],tmp[1])).output().expect("命令执行异常错误提示");
        let ls_la_list = String::from_utf8(output.stdout); 
        match ls_la_list {
            Ok(info) => {
                for x in info.lines() {
                    app.kind_detail.items.push(x.to_string());
                }
            },
            Err(e) => {
                app.kind_detail.items.push(format!("{}",e))
            }
        };
    } 
    // else {
    //     app.kind_detail.items.push(String::from("None"))
    // }

    let items: Vec<ListItem> = app
        .kind_detail.items
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
    f.render_stateful_widget(items, chunks[0],&mut app.kind_detail.state);
}