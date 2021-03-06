// 此示例演示了clap创建参数的完整“生成器模式”样式，该样式
// 相对详细而冗长，但可以更轻松地进行编辑，有时还提供一些更高级的选项，并且
// 支持动态生成参数。
extern crate  clap;
use clap::{App, Arg, SubCommand};

mod host;
use host::*;
use std::thread;
use std::time::Duration;
use std::process;

pub fn clap_init() {
    ctrlc::set_handler(move || {
        println!("Say Good Bye!");
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

	let matches = App::new("My Super Program")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("controls testing features")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg(
                    Arg::with_name("debug")
                        .short("d")
                        .help("print debug information verbosely"),
                ),
        )
        .subcommand(
            SubCommand::with_name("monitor")
                .about("show linux current status")
                .version("0.1")
                .author("Someone E. <someone_else@other.com>")
                .arg(
                    Arg::with_name("lazy")
                        .help("快速查看")
                )
        )
        .get_matches();

    // let matches = clap_app!(myapp =>
    //     (version: "1.0")
    //     (author: "Li")
    //     (about: "Does awesome things")
    //     (@arg CONFIG: -c --config +takes_value "Sets a custom config file")
    //     (@arg INPUT: +required "Sets the input file to use")
    //     (@arg DEBUG: -d ... "Sets the level of debuggind information")
    //     (@arg DETAIL: -v ... "for detail")
    //     (@subcommand test =>
    //         (about: "Controls testing features")
    //         (version: "1.3")
    //         (author: "Li x")
    //         (@arg verbose: -v --verbose "Print test information verbosely")
    //     )
    // ).get_matches();

    // 如果用户提供、则获取该值作为config，或者默认使用 “default.conf”
    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    // 在这里调用.unwrap（）是安全的，因为需要“ INPUT”（如果不需要“ INPUT”，
    // 可以使用 “if let” 有条件地获取值）
    println!("Using input file: {}", matches.value_of("INPUT").unwrap());

    // 根据用户使用“详细”标志的次数来改变输出
    // (比如 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    // 你可以通过以下方式处理有关子命令的信息：按名称请求它们的匹配（如下所示）
    // 仅请求正在使用的名称或两者同时请求
    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }

    if let Some(matches) = matches.subcommand_matches("monitor") {
        if matches.is_present("lazy") {
            loop {
                thread::sleep(Duration::from_secs(1));
                // println!("{}",Local::now())
                monitor()
            }
        }
    }
}