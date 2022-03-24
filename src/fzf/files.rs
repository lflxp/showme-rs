use crate::fzf::App;
use std::{
    process,
    fs::File,
    io::Read,
    path::Path,
};
use tui::{
    widgets::{ListItem, List,Block,Borders, Paragraph},
    style::{Modifier,Color,Style},
    backend::{Backend},
    text::{Span,Spans},
    layout::{Rect, Constraint, Layout, Alignment, Direction},
    Frame
};

pub fn draw_right<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let constraints = if app.show_detail { 
        vec![
            Constraint::Percentage(60),
            Constraint::Percentage(40)
        ]
    } else {
        vec![Constraint::Percentage(100)]
    };

    let chunks = Layout::default()
        .constraints(constraints.as_ref())
        .direction(Direction::Horizontal)
        .split(area);

    app.search.items.clear();
    // app.current = 0;
    // for x in &app.items.items {
    //     if x.contains(&app.input) {
    //         app.search.push(x.to_string());
    //     }
    // }
    for x in &app.files.items {
        if x.contains(&app.input) {
            app.search.items.push(x.to_string())
        }
    }
    let items: Vec<ListItem> = app
        .search
        .items
        .iter()
        .map(|i| {
            ListItem::new(vec![Spans::from(Span::raw(i))])
        })
        .collect();

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("文件或文件夹 [Detail(F1)]"))
        .highlight_style(
            Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    // We can now render the item list
    f.render_stateful_widget(items, chunks[0], &mut app.search.state);
    if app.show_detail {
        ui_text(f,app,chunks[1]);
    }
}

fn ui_text<B: Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    // Words made "loooong" to demonstrate line breaking.
    let s = "Veeeeeeeeeeeeeeeery    loooooooooooooooooong   striiiiiiiiiiiiiiiiiiiiiiiiiing.   ";
    let mut long_line = s.repeat(usize::from(area.width) / s.len() + 4);
    long_line.push('\n');

    let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));
    f.render_widget(block, area);

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

    let mut filename = &String::from("None");
    let contents:String;
    let mut data = Vec::new();
    if app.search.items.len() > 0 {
        filename = app.search.items.get(app.current).unwrap();
        let path = Path::new(filename);
        if !path.exists() {
            println!("Not Found");
            process::exit(1)
        }
        let mut files = File::open(filename).expect("Unable to open file");
        let mut buf = vec![];
        
        // files.read_to_end(&mut buf).expect("uread to end");
        match files.read_to_end(&mut buf) {
            Ok(_) => {
                contents = String::from_utf8_lossy(&buf).to_string();
            },
            Err(e) => {
                contents = String::from(format!("{}", e));
            }
        };
        
        
        for line in contents.lines() {
            data.push(Spans::from(Span::styled(line, Style::default())));
        }
    } else {
        data.push(Spans::from(Span::styled("None", Style::default())))
    }

    let create_block = |title| {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::White).fg(Color::Black))
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            ))
    };
    let paragraph = Paragraph::new(data.clone())
        .style(Style::default().bg(Color::White).fg(Color::Black))
        .block(create_block(filename))
        .alignment(Alignment::Left);
    f.render_widget(paragraph, chunks[0]);
    // let paragraph = Paragraph::new(text.clone())
    //     .style(Style::default().bg(Color::White).fg(Color::Black))
    //     .block(create_block("Left, wrap"))
    //     .alignment(Alignment::Left)
    //     .wrap(Wrap { trim: true });
    // f.render_widget(paragraph, chunks[1]);
    // let paragraph = Paragraph::new(data.clone())
    //     .style(Style::default().bg(Color::White).fg(Color::Black))
    //     .block(create_block("Center, wrap"))
    //     .alignment(Alignment::Left)
    //     .wrap(Wrap { trim: true })
    //     .scroll((app.scroll, 0));
    // f.render_widget(paragraph, chunks[0]);
    // let paragraph = Paragraph::new(text)
    //     .style(Style::default().bg(Color::White).fg(Color::Black))
    //     .block(create_block("Right, wrap"))
    //     .alignment(Alignment::Right)
    //     .wrap(Wrap { trim: true });
    // f.render_widget(paragraph, chunks[3]);
}
