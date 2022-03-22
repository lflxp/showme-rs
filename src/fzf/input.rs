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
use std::{
    error::Error, 
    io::{self, Write, Read}, 
    fs::File,
    time::{Duration, Instant},
    path::Path, process};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, ListState, Tabs, Wrap},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;
use walkdir::{WalkDir,DirEntry};
// https://blog.csdn.net/wsp_1138886114/article/details/116454414?utm_medium=distribute.pc_relevant.none-task-blog-2~default~baidujs_baidulandingword~default-0.pc_relevant_default&spm=1001.2101.3001.4242.1&utm_relevant_index=3

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

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
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
    tabs: TabsState<'a>,
    show_detail: bool, // 显示文件详情
    scroll: u16,
    // events: Vec<(&'a str, &'a str)>,
}

impl<'a> App<'a> {
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
        let finename = String::from(format!("{}/.zsh_history",std::env::home_dir().unwrap().display().to_string()));
        let path = Path::new(&finename);
        if !path.exists() {
            println!("Not Found");
            process::exit(1)
        }
        let mut files = File::open(&finename).expect("Unable to open file");
        let mut buf = vec![];
        files.read_to_end(&mut buf).expect("uread to end");
        let contents = String::from_utf8_lossy(&buf);
        // 数组按倒序排列
        let mut sorted: Vec<&str> = contents.lines().collect(); // contents.split("\n").collect();
        // 倒序
        sorted.reverse();
        // for line in contents.lines() {
        for line in sorted {
            if line.contains(":0;") {
                // 去除无用数据
                self.history.items.push(line[15..line.len()].to_string());
            }
        }
        // println!("{:?}",contents);

        // let output = if cfg!(target_os = "windows") {
        //     Command::new("cmd").arg("/c").arg("dir c:\\").output().expect("cmd exec error!");
        // } else {
        //     Command::new("sh").arg("-c").arg("fc -rl 1").output().expect("sh exec error!");
        // };

        // let output = Command::new("fc").arg("-rl").arg("1").output().expect("sh exec error!");

        // let output_str = String::from_utf8(output.stdout);
        // println!("11111111111 {:?}", output_str);
    }

    fn on_tick(&mut self) {
        self.scroll += 1;
        self.scroll %= 50;
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
            history: StatefulList::with_items(vec![]),
            tabs: TabsState::new(vec!["Files","Command","Core","Host","Env","TODO"]),
            show_detail: false,
            scroll: 0
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
    let tick_rate = Duration::from_millis(250);
    let mut app = App::default();
    app.getfiles2();
    app.gethistory();
    let res = run_app(&mut terminal, &mut app, tick_rate);

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
                        KeyCode::F(1) => {
                            app.show_detail = !app.show_detail;
                        },
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

                            app.tabs.previous();
                        },
                        KeyCode::Right => { 
                            app.tabs.next();
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
        1 => draw_left(f,app,chunks[3]),
        2 => draw_left(f,app,chunks[3]),
        3 => draw_left(f,app,chunks[3]),
        4 => draw_left(f,app,chunks[3]),
        5 => draw_left(f,app,chunks[3]),
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

    let filename = app.search.items.get(app.current).unwrap();
    let path = Path::new(filename);
    if !path.exists() {
        println!("Not Found");
        process::exit(1)
    }
    let mut files = File::open(filename).expect("Unable to open file");
    let mut buf = vec![];
    let contents:String;
    // files.read_to_end(&mut buf).expect("uread to end");
    match files.read_to_end(&mut buf) {
        Ok(_) => {
            contents = String::from_utf8_lossy(&buf).to_string();
        },
        Err(e) => {
            contents = String::from(format!("{}", e));
        }
    };
    
    let mut data = Vec::new();
    for line in contents.lines() {
        data.push(Spans::from(Span::styled(line, Style::default())));
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
