# RUN

> cargo run -- monitor --lazy

```
➜  clapdemo git:(master) ✗ cargo run -- monitor --lazy
   Compiling clapdemo v0.1.0 (/root/code/clapdemo)
    Finished dev [unoptimized + debuginfo] target(s) in 1.79s
     Running `target/debug/clapdemo monitor --lazy`
Value for config: default.conf
No verbose info
     Uptime       : 0 days
     Hostname     : DESKTOP-DVBN0M3
     Load         : 0.43 0.59 0.80
     DISK         : Total 250 Used 65 Free 172 Percent 27.60
     MEM          : Total 25 Available 16 Used 7 Free 10 Percent 27.60
     SWAP         : Total 7 Used 0 Free 6 Percent 0.00
  oops
-------- -----load-avg---- ----cpu-usage--- ---swap--- ----net(A)---- 
  time  |  1m    5m   15m | usr sys idl iow|   si   so|   recv   send|
02:19:17| 0.43  0.59  0.80|   1   1  95   0|    0    1|      0      0|
02:19:18| 0.43  0.59  0.80|   0   0  98   0|    0    0|     1k     3k|
02:19:19| 0.47  0.60  0.80|   0   0  99   0|    0    0|    216     1k|
02:19:20| 0.47  0.60  0.80|   0   0  99   0|    0    0|     1k     2k|
02:19:21| 0.47  0.60  0.80|   0   0  99   0|    0    0|    454     2k|
02:19:22| 0.47  0.60  0.80|   0   0  98   0|    0    0|     4k    63k|
02:19:23| 0.47  0.60  0.80|   1   1  96   0|    0    0|    908     3k|
02:19:24| 0.84  0.67  0.83|   0   0  99   0|    0    0|     7k     5k|
02:19:25| 0.84  0.67  0.83|   0   0  98   0|    0    0|     3k     7k|
02:19:26| 0.84  0.67  0.83|   0   0  98   0|    0    0|    508     2k|
02:19:27| 0.84  0.67  0.83|   0   0  99   0|    0    0|    289     1k|
02:19:28| 0.84  0.67  0.83|   0   0  99   0|    0    0|     2k     4k|
02:19:29| 0.77  0.66  0.82|   0   0  98   0|    0    0|     6k     6k|
02:19:30| 0.77  0.66  0.82|   0   0  99   0|    0    0|     1k     4k|
02:19:31| 0.77  0.66  0.82|   2   5  91   0|    0    0|     2k    14k|
02:19:32| 0.77  0.66  0.82|   2  74   0   0|    0    0|     2k     5k|
02:19:33| 0.77  0.66  0.82|   2  74   0   0|    0    0|     1k     3k|
02:19:34| 3.03  1.13  0.97|   2  73   0   0|    0    0|     2k     4k|
02:19:35| 3.03  1.13  0.97|   1  74   0   0|    0    0|     4k     7k|
-------- -----load-avg---- ----cpu-usage--- ---swap--- ----net(A)---- 
  time  |  1m    5m   15m | usr sys idl iow|   si   so|   recv   send|
02:19:37| 3.03  1.13  0.97|   2  73   0   0|    0    0|     2k     4k|
02:19:38| 3.03  1.13  0.97|   2  75   0   0|    0    0|    981     3k|
02:19:39| 5.03  1.58  1.12|   2  74   0   0|    0    0|    11k     5m|
02:19:40| 5.03  1.58  1.12|   1  74   0   0|    0    0|    26k    15m|
02:19:41| 5.03  1.58  1.12|   1  76   0   0|    0    0|     6k     2m|
02:19:42| 5.03  1.58  1.12|   1  73   0   0|    0    0|    48k    23m|
02:19:43| 5.03  1.58  1.12|   2  74   0   0|    0    0|    20k    11m|
02:19:44| 5.99  1.83  1.20|   1  71   0   0|    0    0|    47k    18m|
02:19:45| 5.99  1.83  1.20|   2  73   0   0|    0    0|    55k    17m|
02:19:46| 5.99  1.83  1.20|   1  72   0   0|    0    0|    49k    12m|
02:19:47| 5.99  1.83  1.20|   2  73   0   0|    0    0|    55k    20m|
02:19:48| 5.99  1.83  1.20|   2  73   0   0|    0    0|    51k    20m|
02:19:49| 7.75  2.27  1.35|   1  72   0   0|    0    0|    52k    17m|
02:19:50| 7.75  2.27  1.35|   1  73   0   0|    0    0|    42k    18m|
02:19:51| 7.75  2.27  1.35|   1  72   0   0|    0    0|    30k    14m|
02:19:52| 7.75  2.27  1.35|   1  73   0   0|    0    0|     2k     4k|
02:19:53| 7.75  2.27  1.35|   2  76   0   0|    0    0|     2k     4k|
^CSay Good Bye!
```

## fzf

```
 Press Esc to stop editing, Enter to record the message 41/9842 3
  ┌功能项─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
  │ Files │ Command │ Core │ Host │ Env │ TODO                                                                                                                                                                │
  └───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘
  ┌搜索框─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
  │.rs                                                                                                                                                                                                        │
  └───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘
  ┌文件或文件夹─────────────────────────────────────────────────────────────────────────────────────────────────────────────┐┌./src/colorrun.rs───────────────────────────────────────────────────────────────┐
  │  ./src/c2.rs                                                                                                            ││use log::{info};                                                                │
  │  ./src/clapsed.rs                                                                                                       ││                                                                                │
  │  ./src/color.rs                                                                                                         ││extern crate colored; // not needed in Rust 2018                                │
  │> ./src/colorrun.rs                                                                                                      ││                                                                                │
  │  ./src/host/all.rs                                                                                                      ││use colored::*;                                                                 │
  │  ./src/host/mod.rs                                                                                                      ││                                                                                │
  │  ./src/host/sys/linux/hostname.rs                                                                                       ││// 这个函数仅当操作系统是 Linux 的时候才会编译                                  │
  │  ./src/host/sys/linux/mod.rs                                                                                            ││#[cfg(target_os = "linux")]                                                     │
  │  ./src/host/sys/macos/hostname.rs                                                                                       ││fn are_you_on_linux() {                                                         │
  │  ./src/host/sys/macos/mod.rs                                                                                            ││    println!("You are running linux!")                                          │
  │  ./src/host/sys/mod.rs                                                                                                  ││}                                                                               │
  │  ./src/scan/mod.rs                                                                                                      ││                                                                                │
  │  ./src/scan/output.rs                                                                                                   ││// 而这个函数仅当操作系统**不是** Linux 时才会编译                              │
  │  ./src/scan/parse.rs                                                                                                    ││#[cfg(not(target_os = "linux"))]                                                │
  └─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘└────────────────────────────────────────────────────────────────────────────────┘
```


```
use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};

fn main(){
    loop {
        // use the `>` character as the prompt
        // need to explicitly flush this to ensure it prints before read_line
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // read_line leaves a trailing newline, which trim removes
        // this needs to be peekable so we can determine when we are on the last command
        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next()  {

            // everything after the first whitespace character is interpreted as args to the command
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
                    // default to '/' as new directory if one was not provided
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }

                    previous_command = None;
                },
                "exit" => return,
                command => {
                    let stdin = previous_command
                        .map_or(Stdio::inherit(),
                                |output: Child| Stdio::from(output.stdout.unwrap()));

                    let stdout = if commands.peek().is_some() {
                        // there is another command piped behind this one
                        // prepare to send output to the next command
                        Stdio::piped()
                    } else {
                        // there are no more commands piped behind this one
                        // send output to shell stdout
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => { previous_command = Some(output); },
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        },
                    };
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            // block until the final command has finished
            final_command.wait().unwrap();
        }

    }
}
```

# TODO

1. [x] blackwater ip scan
2. gui
3. static http server
4. gotty
5. mqtt => https://zhuanlan.zhihu.com/p/264181114 => https://gitee.com/lflxp/mqtt-example-rust
6. git log + git show + git reset + git stash/status + rs popus
7. kubectl get po + kubectl edit + $*
8. rust shell => https://www.cnblogs.com/ishenghuo/p/12550142.html

# BUG

1. HOME/END