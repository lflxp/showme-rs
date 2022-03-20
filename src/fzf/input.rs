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
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io::{self, Write, Read}, path::Path, process};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, ListState},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;
use walkdir::{WalkDir,DirEntry};
// https://blog.csdn.net/wsp_1138886114/article/details/116454414?utm_medium=distribute.pc_relevant.none-task-blog-2~default~baidujs_baidulandingword~default-0.pc_relevant_default&spm=1001.2101.3001.4242.1&utm_relevant_index=3

// pub async fn testfile() -> Result<Vec<String>,()> {
    // let mut results = Vec::new();
	// for entry in WalkDir::new("./").into_iter().filter_map(|e| e.ok()) {
    //     // println!("{}", entry.path().display());
    //     results.push(entry.path().display().to_string());
    // }

    // let mut results = WalkDir::new("./")
    //     .into_iter()
    //     .filter_map(|e| {
    //         if e.ok() {
    //             results.push(e.path().display().to_string().as_str());
    //         }
    //     });

    // WalkDir::new("./")
    //     .into_iter()
    //     .filter_entry(|e|is_not_hidden(e))
    //     .filter_map(|v|v.ok())
    //     .for_each(|x|{
    //         results.push(x.clone().path().display().to_string().as_str())
    //     });
    // return results;
// }

fn is_not_hidden(entry:&DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        // .map(|s| !s.ends_with(".jpg"))
        .map(|s| !s.starts_with(".git"))
        .unwrap_or(false)
}

enum InputMode {
    Normal,
    Editing,
}

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

/// App holds the state of the application
struct App<'a> {
    /// Current value of the input box
    input: String,
    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    messages: Vec<String>,
    search: StatefulList<String>,
    current: usize,
    items: StatefulList<&'a str>,
    files: StatefulList<String>,
    history: StatefulList<String>,
    // events: Vec<(&'a str, &'a str)>,
}

// const TASKS: [&str; 24] = [
//     "Item1", "Item2", "Item3", "Item4", "Item5", "Item6", "Item7", "Item8", "Item9", "Item10",
//     "Item11", "Item12", "Item13", "Item14", "Item15", "Item16", "Item17", "Item18", "Item19",
//     "Item20", "Item21", "Item22", "Item23", "Item24",
// ];

impl<'a> App<'a> {
    // pub fn getfiles(&self) -> Result<Vec<String>,()> {
    //     let data: Vec<DirEntry> = WalkDir::new("./").into_iter().filter_map(|e| e.ok()).map(|x|x).collect();

    //     let mut result = Vec::new();
    //     for entry in data.clone() {
    //         result.push(entry.path().display().to_string())
    //     }
    //     Ok(result)
    // }

    pub fn getfiles2(&mut self) {
        // let info = self.getfiles().unwrap();
        // for x in info.iter().enumerate() {
        //     self.files.items.insert(x.0, x.1.to_string());
        //     // self.items.items.insert(x.0, x.1.as_str());
        // }

        WalkDir::new("./")
            .into_iter()
            .filter_entry(|e|is_not_hidden(e)) // 排除
            .filter_map(|e| e.ok())
            .for_each(|x| {
                // self.files.items.insert(x.0, x.1.path().display().to_string())
                self.files.items.push(x.path().display().to_string())
            })
    }

    // https://www.twle.cn/c/yufei/rust/rust-basic-file-input-output.html
    pub fn gethistory(&mut self) {
        let finename = String::from("/home/lxp/.zsh_history");
        let path = Path::new(&finename);
        if !path.exists() {
            println!("Not Fount");
            process::exit(1)
        }
        let mut files = std::fs::File::open(&finename).expect("Unable to open file");
        let mut contents = Vec::new();
        files.read_to_end(&mut contents).unwrap();
        // for x in contents.lines() {
        //     self.history.items.push(x.to_string());
        // }
        println!("{:?}",contents);
    }
}

impl<'a> Default for App<'a> {
    fn default() -> App<'a> {
        App {
            input: String::new(),
            input_mode: InputMode::Editing,
            messages: Vec::new(),
            items: StatefulList::with_items(vec![]),
            // items: StatefulList::with_items(TASKS.to_vec()),
            current: 0,
            search: StatefulList::with_items(vec![]),
            files: StatefulList::with_items(vec![]),
            history: StatefulList::with_items(vec![])
            // events: vec![
            //     ("Event1", "INFO"),
            //     ("Event2", "INFO"),
            //     ("Event3", "CRITICAL"),
            //     ("Event4", "ERROR"),
            //     ("Event5", "INFO"),
            //     ("Event6", "INFO"),
            //     ("Event7", "WARNING"),
            //     ("Event8", "INFO"),
            //     ("Event9", "INFO"),
            //     ("Event10", "INFO"),
            //     ("Event11", "CRITICAL"),
            //     ("Event12", "INFO"),
            //     ("Event13", "INFO"),
            //     ("Event14", "INFO"),
            //     ("Event15", "INFO"),
            //     ("Event16", "INFO"),
            //     ("Event17", "ERROR"),
            //     ("Event18", "ERROR"),
            //     ("Event19", "INFO"),
            //     ("Event20", "INFO"),
            //     ("Event21", "WARNING"),
            //     ("Event22", "INFO"),
            //     ("Event23", "INFO"),
            //     ("Event24", "WARNING"),
            //     ("Event25", "INFO"),
            //     ("Event26", "INFO"),
            // ],
        }
    }
}

pub fn run_input() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::default();
    app.getfiles2();
    app.gethistory();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // 显示输出
    let info = app.search.items.get(app.current).unwrap();
    io::stdout().write_all(info.as_bytes())?;
    
    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

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
                    KeyCode::Left => { 
                        app.search.unselect();
                        match app.search.state.selected() {
                            Some(i) => {
                                app.current = i;
                            }
                            None => app.current = 0
                        }
                    },
                    KeyCode::Down => { 
                        app.search.next();
                        match app.search.state.selected() {
                            Some(i) => {
                                app.current = i;
                            }
                            None => app.current = 0
                        }
                    },
                    KeyCode::Up => {
                        app.search.previous();
                        match app.search.state.selected() {
                            Some(i) => {
                                app.current = i;
                            }
                            None => app.current = 0
                        }
                    },
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
                        return Ok(());
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                        app.search.items.clear();
                        app.current = 0;
                        app.search.unselect();
                        // app.files.unselect();
                        for x in &app.files.items {
                            if x.contains(&app.input) {
                                // app.search.items.insert(x.0,x.1.to_string());
                                app.search.items.push(x.to_string());
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Left => { 
                        app.search.unselect();
                        match app.search.state.selected() {
                            Some(i) => {
                                app.current = i;
                            }
                            None => app.current = 0
                        }
                    },
                    KeyCode::Down => { 
                        app.search.next();
                        match app.search.state.selected() {
                            Some(i) => {
                                app.current = i;
                            }
                            None => app.current = 0
                        }
                    },
                    KeyCode::Up => {
                        app.search.previous();
                        match app.search.state.selected() {
                            Some(i) => {
                                app.current = i;
                            }
                            None => app.current = 0
                        }
                    },
                    _ => {}
                },
            }
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
                Span::raw(" to start editing. "),
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
                Span::raw(" to record the message "),
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
    f.render_widget(input, chunks[1]);
    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[1].x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[1].y + 1,
            )
        }
    }

    
    // f.render_widget(messages, chunks[2]);
    draw_message(f,app,chunks[2]);
}

fn draw_message<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ])
        .direction(Direction::Horizontal)
        .split(area);
    draw_left(f,app,chunks[1]);
    draw_right(f,app,chunks[0]);
}

fn draw_left<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    // let messages: Vec<ListItem> = app
    //     .messages
    //     .iter()
    //     .enumerate()
    //     .map(|(i, m)| {
    //         let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
    //         ListItem::new(content)
    //     })
    //     .collect();

    let messages: Vec<ListItem> = app
        .history
        .items
        .iter()
        .map(|x| {
            ListItem::new(vec![Spans::from(Span::raw(x))]) 
        })
        .collect();
    let messages =
        List::new(messages).block(Block::default().borders(Borders::ALL).title("历史命令"));
    f.render_widget(messages, area)
}

fn draw_right<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    // let messages: Vec<ListItem> = app
    //     .messages
    //     .iter()
    //     .enumerate()
    //     .map(|(i, m)| {
    //         let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
    //         ListItem::new(content)
    //     })
    //     .collect();
    // let messages =
    //     List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
    // f.render_widget(messages, area);
    app.search.items.clear();
    // app.current = 0;
    // for x in &app.items.items {
    //     if x.contains(&app.input) {
    //         app.search.push(x.to_string());
    //     }
    // }
    for x in &app.files.items {
        if x.contains(&app.input) {
            app.search.items.insert(0,x.to_string())
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
    // let items: Vec<ListItem> = app
    //     .items
    //     .items
    //     .iter()
    //     .map(|i| {
    //         // let mut lines = vec![Spans::from(i.0)];
    //         // for _ in 0..i.1 {
    //         //     lines.push(Spans::from(Span::styled(
    //         //         "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
    //         //         Style::default().add_modifier(Modifier::ITALIC),
    //         //     )));
    //         // }
    //         // ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
    //         ListItem::new(vec![Spans::from(Span::raw(*i))])
    //     })
    //     .collect();

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("文件或文件夹"))
        .highlight_style(
            Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    // We can now render the item list
    f.render_stateful_widget(items, area, &mut app.search.state);
}