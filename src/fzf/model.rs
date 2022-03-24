use tui::{
    widgets::{ListState}
};

use std::{
    fs::File,
    io::Read,
    path::Path,
    process::{self, Command}
};
use walkdir::{WalkDir,DirEntry};

pub enum InputMode {
    Normal,
    Editing,
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
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

    pub fn previous(&mut self) {
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

    pub fn unselect(&mut self) {
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

fn is_not_hidden(entry:&DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        // .map(|s| !s.ends_with(".jpg"))
        .map(|s| !s.starts_with(".git"))
        .unwrap_or(false)
}


/// App holds the state of the application
pub struct App<'a> {
    /// Current value of the input box
    pub input: String,
    /// Current input mode
    pub input_mode: InputMode,
    /// History of recorded messages
    pub messages: Vec<String>,
    pub search: StatefulList<String>,
    pub current: usize,
    // items: StatefulList<&'a str>,
    pub files: StatefulList<String>,
    pub history: StatefulList<String>,
    pub history_search: StatefulList<String>,
    pub gits: StatefulList<String>,
    pub gits_search: StatefulList<String>,
    pub tabs: TabsState<'a>,
    pub show_detail: bool, // 显示文件详情
    pub scroll: u16,
    pub gits_detail: StatefulList<String>,
    pub currentdetail: usize,
    pub show_popup: bool,
    pub kinddetail: usize,
    pub kind: String,
    pub kind_input: bool,
    pub kind_data: StatefulList<String>, 
    pub kind_search: StatefulList<String>, 
    pub kind_detail: StatefulList<String>, 
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

    pub fn get_git(&mut self) {
        // let output = if cfg!(target_os = "windows") {
        //     Command::new("cmd").arg("/c").arg("git log --pretty=format:'%h %s'").output().expect("cmd exec error!");
        // } else {
        //     Command::new("sh").arg("-c").arg("git log --pretty=format:'%h %s'").output().expect("sh exec error!");
        // };

        let output = Command::new("sh").arg("-c").arg("git log --pretty=format:'%h %s %cr'").output().expect("命令执行异常错误提示");
        let ls_la_list = String::from_utf8(output.stdout);
        // println!("{:?}",ls_la_list);
        match ls_la_list {
            Ok(info) => {
                for x in info.lines() {
                    self.gits.items.push(x.to_string());
                }
            },
            Err(e) => {
                self.gits.items.push(format!("{}", e));
                eprintln!("{}", e)
            }
        }
    }

    pub fn on_tick(&mut self) {
        self.scroll += 1;
        self.scroll %= 50;
    }
}

impl<'a> Default for App<'a> {
    fn default() -> App<'a> {
        App {
            show_popup: false,
            input: String::new(),
            input_mode: InputMode::Editing,
            messages: Vec::new(),
            // items: StatefulList::with_items(vec![]),
            // items: StatefulList::with_items(TASKS.to_vec()),
            current: 0,
            search: StatefulList::with_items(vec![]),
            files: StatefulList::with_items(vec![]),
            history: StatefulList::with_items(vec![]),
            history_search: StatefulList::with_items(vec![]),
            tabs: TabsState::new(vec!["Files(F2)","Command(F3)","Git(F4)","Kubectl(F5)","Env","TODO"]),
            show_detail: false,
            scroll: 0,
            gits: StatefulList::with_items(vec![]),
            gits_search: StatefulList::with_items(vec![]),
            gits_detail: StatefulList::with_items(vec![]),
            currentdetail: 0,
            kinddetail: 0,
            kind_input: false,
            kind: String::from("po"),
            kind_data: StatefulList::with_items(vec![]),
            kind_search: StatefulList::with_items(vec![]),
            kind_detail: StatefulList::with_items(vec![]),
        }
    }
}