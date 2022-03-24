/// A simple example demonstrating how to handle user input. This is
/// a bit out of the scope of the library as it does not provide any
/// input handling out of the box. However, it may helps some to get
/// started.
///
/// This is a very simple example:
///   * A input box always focused. Every character you type is registered
///   here
///   * Pressing Backspace erases a character
///   * Pressing Enter pushes the current input in the history of previous
///   messages
use super::*;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error, 
    io::{self, Write}, 
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

// https://blog.csdn.net/wsp_1138886114/article/details/116454414?utm_medium=distribute.pc_relevant.none-task-blog-2~default~baidujs_baidulandingword~default-0.pc_relevant_default&spm=1001.2101.3001.4242.1&utm_relevant_index=3

pub fn run_input() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let mut app = App::default();
    app.getfiles2();
    app.gethistory();
    app.get_git();
    let res = run_app(&mut terminal, &mut app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // 获取回车信息
    let mut info:&String = &String::from("");
    // 显示输出
    if app.tabs.index == 0 {
        info = app.search.items.get(app.current).unwrap();
    } else if app.tabs.index == 1 {
        info = app.history_search.items.get(app.current).unwrap();
    }
    
    io::stdout().write_all(info.as_bytes())?;
    
    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: &mut App, tick_rate: Duration) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(10));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match app.input_mode {
                    // 按键捕获
                    InputMode::Normal => match key.code {
                        KeyCode::Char('e') => {
                            app.input_mode = InputMode::Editing;
                        }
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        // KeyCode::Left => { 
                        //     app.search.unselect();
                        //     match app.search.state.selected() {
                        //         Some(i) => {
                        //             app.current = i;
                        //         }
                        //         None => app.current = 0
                        //     }
                        // },
                        // KeyCode::Down => { 
                        //     app.search.next();
                        //     match app.search.state.selected() {
                        //         Some(i) => {
                        //             app.current = i;
                        //         }
                        //         None => app.current = 0
                        //     }
                        // },
                        // KeyCode::Up => {
                        //     app.search.previous();
                        //     match app.search.state.selected() {
                        //         Some(i) => {
                        //             app.current = i;
                        //         }
                        //         None => app.current = 0
                        //     }
                        // },
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => {
                            app.messages.push(app.input.drain(..).collect());

                            // let info = app.search.items.get(app.current).unwrap();
                            // app.messages.push(info.to_string());

                            // https://www.twle.cn/c/yufei/rust/rust-basic-input-output.html
                            // io::stdout().write_all(info.as_bytes())?;

                            // std::io::stdout().write(format!("\n写入的字节数为：{}",100).as_bytes()).unwrap();
                            // println!("1111111111111111 {}",info);

                            // process::exit(0);
                            if app.tabs.index == 2 {
                                app.show_popup = !app.show_popup;
                            } else if  app.tabs.index == 3 {
                                app.kind_input = !app.kind_input;
                            } else {
                                return Ok(());
                            }
                        }
                        KeyCode::F(1) => {
                            app.show_detail = !app.show_detail;
                        },
                        KeyCode::F(2) => {
                            app.current = 0;
                            app.tabs.index = 0;
                        },
                        KeyCode::F(3) => {
                            app.current = 0;
                            app.tabs.index = 1;
                        },
                        KeyCode::F(4) => {
                            app.current = 0;
                            app.tabs.index = 2;
                        },
                        KeyCode::F(5) => {
                            app.current = 0;
                            app.tabs.index = 3;
                        },
                        KeyCode::Char(c) => {
                            app.input.push(c);
                            app.current = 0;
                            if app.tabs.index == 0 {
                                app.search.items.clear();
                                app.search.unselect();
                                // app.files.unselect();
                                for x in &app.files.items {
                                    if x.contains(&app.input) {
                                        // app.search.items.insert(x.0,x.1.to_string());
                                        app.search.items.push(x.to_string());
                                    }
                                }
                            } else if app.tabs.index == 1 {
                                app.history_search.items.clear();
                                app.history_search.unselect();
                                for x in &app.history.items {
                                    if x.contains(&app.input) {
                                        // app.search.items.insert(x.0,x.1.to_string());
                                        app.history_search.items.push(x.to_string());
                                    }
                                }
                            } else if app.tabs.index == 2 {
                                app.gits_search.items.clear();
                                app.gits_search.unselect();
                                for x in &app.gits.items {
                                    if x.contains(&app.input) {
                                        // app.search.items.insert(x.0,x.1.to_string());
                                        app.gits_search.items.push(x.to_string());
                                    }
                                }
                            } else if app.tabs.index == 3 {
                                app.kind_search.items.clear();
                                app.kind_search.unselect();
                                for x in &app.kind_data.items {
                                    if x.contains(&app.input) {
                                        // app.search.items.insert(x.0,x.1.to_string());
                                        app.kind_search.items.push(x.to_string());
                                    }
                                }
                            }
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                        }
                        KeyCode::Esc => {
                            app.input_mode = InputMode::Normal;
                        },
                        KeyCode::Home => {
                            if app.tabs.index == 2 {
                                app.gits_detail.previous();
                                match app.gits_detail.state.selected() {
                                    Some(i) => {
                                        app.currentdetail = i;
                                    }
                                    None => app.currentdetail = 0
                                }
                            } else if app.tabs.index == 3 {
                                app.kind_detail.previous();
                                match app.kind_detail.state.selected() {
                                    Some(i) => {
                                        app.kinddetail = i;
                                    }
                                    None => app.kinddetail = 0
                                }
                            }
                        },
                        KeyCode::End => {
                            if app.tabs.index == 2 {
                                app.gits_detail.next();
                                match app.gits_detail.state.selected() {
                                    Some(i) => {
                                        app.currentdetail = i;
                                    }
                                    None => app.currentdetail = 0
                                }
                            } else if app.tabs.index == 3 {
                                app.kind_detail.next();
                                match app.kind_detail.state.selected() {
                                    Some(i) => {
                                        app.kinddetail = i;
                                    }
                                    None => app.kinddetail = 0
                                }
                            }
                        },
                        KeyCode::Left => { 
                            app.search.unselect();
                            match app.search.state.selected() {
                                Some(i) => {
                                    app.current = i;
                                }
                                None => app.current = 0
                            }

                            app.tabs.previous();
                        },
                        KeyCode::Right => { 
                            app.tabs.next();
                        },
                        KeyCode::Down => { 
                            if app.tabs.index == 0 {
                                app.search.next();
                                match app.search.state.selected() {
                                    Some(i) => {
                                        app.current = i;
                                    }
                                    None => app.current = 0
                                }
                            } else if app.tabs.index == 1 {
                                app.history_search.next();
                                match app.history_search.state.selected() {
                                    Some(i) => {
                                        app.current = i;
                                    }
                                    None => app.current = 0
                                }
                            } else if app.tabs.index == 2 {
                                app.currentdetail = 0;
                                app.gits_search.next();
                                match app.gits_search.state.selected() {
                                    Some(i) => {
                                        app.current = i;
                                    }
                                    None => app.current = 0
                                }
                            } else if app.tabs.index == 3 {
                                app.kinddetail = 0;
                                app.kind_search.next();
                                match app.kind_search.state.selected() {
                                    Some(i) => {
                                        app.current = i;
                                    }
                                    None => app.current = 0
                                }
                            }
                        },
                        KeyCode::Up => {
                            if app.tabs.index == 0 {
                                app.search.previous();
                                match app.search.state.selected() {
                                    Some(i) => {
                                        app.current = i;
                                    }
                                    None => app.current = 0
                                }
                            } else if app.tabs.index == 1 {
                                app.history_search.previous();
                                match app.history_search.state.selected() {
                                    Some(i) => {
                                        app.current = i;
                                    }
                                    None => app.current = 0
                                }
                            } else if app.tabs.index == 2 {
                                app.currentdetail = 0;
                                app.gits_search.previous();
                                match app.gits_search.state.selected() {
                                    Some(i) => {
                                        app.current = i;
                                    }
                                    None => app.current = 0
                                }
                            } else if app.tabs.index == 3 {
                                app.kinddetail = 0;
                                app.kind_search.previous();
                                match app.kind_search.state.selected() {
                                    Some(i) => {
                                        app.current = i;
                                    }
                                    None => app.current = 0
                                }
                            } 
                        },
                        _ => {}
                    },
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // ui布局 layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing, F1 to show detail window, Home|End with detail Up and Down, F2|F3|F4 change tab shortKey."),
                Span::raw(format!("{:?}/{:?} {:?} ",app.search.items.len(),app.files.items.len(),app.current)),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message, F1 to show detail window, Home|End with detail Up and Down, F2|F3|F4 change tab shortKey "),
                Span::raw(format!("{:?}/{:?} {:?} ",app.search.items.len(),app.files.items.len(),app.current)),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("搜索框"));
    f.render_widget(input, chunks[2]);
    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[2].x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[2].y + 1,
            )
        }
    }

    draw_tabs(f,app,chunks[1]);
    
    // f.render_widget(messages, chunks[2]);
    // draw_message(f,app,chunks[3]);

    match app.tabs.index {
        0 => draw_right(f,app,chunks[3]),
        1 => draw_command(f,app,chunks[3]),
        2 => draw_commit(f,app,chunks[3]),
        3 => draw_k8s(f,app,chunks[3]),
        4 => draw_command(f,app,chunks[3]),
        5 => draw_command(f,app,chunks[3]),
        _ => unreachable!("") 
    };
}

fn draw_tabs<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| {
            let (first,rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(first,Style::default().fg(Color::Yellow)),
                Span::styled(rest,Style::default().fg(Color::Green)),
            ])
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("功能项"))
        .select(app.tabs.index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        );
    f.render_widget(tabs, area);
}

// fn draw_message<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
//     let chunks = Layout::default()
//         .constraints([
//             Constraint::Percentage(50),
//             Constraint::Percentage(50)
//         ])
//         .direction(Direction::Horizontal)
//         .split(area);
//     draw_left(f,app,chunks[1]);
//     draw_right(f,app,chunks[0]);
// }

