extern crate  clap;
use clap::{App, Arg, SubCommand};

mod host;
use host::*;
use std::thread;
use std::time::Duration;
use std::process;

mod scan;
use scan::{Parseip, pingmethod,print_result, Core, LOGO};
use std::io::Error;
use stopwatch::{Stopwatch};

use std::path::PathBuf;
use log::{debug, error, info, warn};
use log4rs;

mod server;
use server::{server1,server_tokio,server_async};

#[tokio::main]
async fn main() -> Result<(), Error> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

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
            SubCommand::with_name("scan")
                .about("扫描IP和端口")
                .version("0.1")
                .author("lflxp <382023823@qq.com>")
                .arg(
                    Arg::with_name("ip")
                        .short("i")
                        .long("ip")
                        .value_name("IP")
                        .takes_value(true)
                        .help("10.1-100.2.1-255"),
                )
                .arg(
                    Arg::with_name("port")
                        .short("p")
                        .long("port")
                        .value_name("PORT")
                        .takes_value(true)
                        .help("21,22,23,25,69,79,80-89,110,111,113,115,119,135,137,138,139,143,152,153,158,161,162,179,194,201,209,213,218,220,259,264,308,389,443,445,512,513,514,524,530,531,532,540,542,544,546,547,548,554,556,563,591,593,604,631,636,647,648,652,665,666,674,691,692,695,699,700,701,702,706,771,720,749,782,829,860,873,901,902,904,981,989,990,991,992,993,995,1025,1433,1521,2082,2083,2086,2087,2095,2096,2077,2078,2222,2601,2604,3128,3306,3311,3312,3389,5432,5560,5900,5984,6379,7001,7002,7778,8080-9090,9200,9300,9418,10000,11211,27017,27018,50000,50030,50070"),
                )
                .arg(
                    Arg::with_name("concurrency")
                        .short("c")
                        .long("concurrency")
                        .value_name("Concurrency")
                        .takes_value(true)
                        .default_value("65535")
                        .help("超时时间"),
                )
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .long("file")
                        .value_name("FILE")
                        .takes_value(true)
                        .default_value("./demo")
                        .help("file"),
                )
                .arg(
                    Arg::with_name("timeout")
                        .short("t")
                        .long("timeout")
                        .value_name("TIMEOUT")
                        .takes_value(true)
                        .help("超时时间"),
                )
                .arg(
                    Arg::with_name("both")
                        .short("b")
                        .long("both")
                        .help("是否同时进行ip扫描和端口扫描"),
                )
        )
        .subcommand(
            SubCommand::with_name("monitor")
                .about("show linux current status")
                .version("0.1")
                .author("Someone E. <someone_else@other.com>")
                .arg(
                    Arg::with_name("intervals")
                        .short("i")
                        .long("intervals")
                        .help("刷新间隔")
                        .default_value("1")
                        .takes_value(true),
                ) 
                .arg(
                    Arg::with_name("lazy")
                        .long("lazy")
                        .help("快速查看")
                )
                .arg(
                    Arg::with_name("cpu")
                        .short("c")
                        .long("cpu")
                        .help("cpu使用率")
                )
                .arg(
                    Arg::with_name("load")
                        .short("l")
                        .long("load")
                        .help("cpu负载")
                )
                .arg(
                    Arg::with_name("swap")
                        .short("s")
                        .long("swap")
                        .help("swap使用")
                )
                .arg(
                    Arg::with_name("net")
                        .short("n")
                        .long("snet")
                        .help("网络负载")
                )
                .arg(
                    Arg::with_name("Net")
                        .short("N")
                        .long("nets")
                        .help("详细网络负载")
                )
                .arg(
                    Arg::with_name("disk")
                        .short("d")
                        .long("disk")
                        .help("磁盘负载")
                )
        )
        .subcommand(
            SubCommand::with_name("server")
                .about("http server")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg(
                    Arg::with_name("port")
                        .short("p")
                        .long("port")
                        .default_value("9999")
                        .takes_value(true)
                        .help("http server on listen port"),
                )
                .arg(
                    Arg::with_name("host")
                        .short("h")
                        .long("host")
                        .default_value("127.0.0.1")
                        .takes_value(true)
                        .help("http server on listen host"),
                )
                .arg(
                    Arg::with_name("type")
                        .short("t")
                        .long("type")
                        .default_value("spawn")
                        .takes_value(true)
                        .help("http server类型：1.spawn 2.tokio 3.async-std"),
                ),
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
    // .arg(
    //     Arg::with_name("INPUT")
    //         .help("Sets the input file to use")
    //         // .required(true)
    //         .index(1),
    // )
    // println!("Using input file: {}", matches.value_of("INPUT").unwrap());

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

    if let Some(matches) = matches.subcommand_matches("server") {
        let mut port: u16 = 0;
        let mut host: &str = "";
        if matches.is_present("port") {
            port = matches.value_of("port").unwrap().parse::<u16>().unwrap();
        }
        if matches.is_present("host") {
            host = matches.value_of("host").unwrap();
        }

        match matches.value_of("type").unwrap() {
            "spawn" => {
                debug!("spawn");
                server1(host,port)
            },
            "tokio" => {
                debug!("tokio");
                server_tokio(host,port).await
            },
            "async-std" => {
                debug!("async-std");
                server_async(host,port).await
            },
            _ => warn!("unknown type {}", matches.value_of("type").unwrap())
        };

        info!("wahaha")
    }

    if let Some(matches) = matches.subcommand_matches("scan") {
        println!("{}",LOGO);
        let mut ip: &str ="127.0.0.1";
        let mut port: &str = "22,3306";
        let mut timeout: u64 = 1;
        let mut concurrency: u32 = 65535; 
        let mut file: &str = "";
        let mut both: bool = false;
        if matches.is_present("ip") {
            ip = matches.value_of("ip").unwrap();
        }
        if matches.is_present("port") {
            port = matches.value_of("port").unwrap();
        }
        if matches.is_present("timeout") {
            timeout = matches.value_of("timeout").unwrap().parse::<u64>().unwrap();
        }
        if matches.is_present("concurrency") {
            concurrency = matches.value_of("concurrency").unwrap().parse::<u32>().unwrap();
        }
        if matches.is_present("file") {
            file = matches.value_of("file").unwrap();
        }
        if matches.is_present("both") {
            both = true;
        }

        let instance = Parseip{
            ip: String::from(ip),
            port: String::from(port),
            timeout: timeout,
            udp: false,
            concurrency: concurrency,
            outfile: Some(PathBuf::from(file)),
        };

        let ips = instance.get_ips().await.unwrap();
        if ips.len() == 0 {
            panic!("Parameter Error");
        }

        let ports = instance.get_ports().await.unwrap();
        if ports.len() == 0 {
            panic!("Parameter Error");
        }

        let start = Stopwatch::start_new();

        // ping ip 获取有效ip
        match pingmethod(ips).await {
            Ok(data) => {
                if both {
                    let mut core = Core::new(&instance).await;
                    for (index,ip) in data.iter().enumerate() {
                        warn!("Index {} IP {} scanning", index, ip);
                        match core.runasip(ports.clone(), ip.to_string()).await {
                            Ok(_) => debug!("{} ip success",ip),
                            _ => {}
                        }
                    }
                }
            }
            Err(e) => error!("{}", e),
        }
        
        print_result(start).await;
    }

    if let Some(matches) = matches.subcommand_matches("monitor") {
        let mut args = Vec::new();
        if matches.is_present("lazy") {
            args.push("--cpu");
            args.push("--load");
            args.push("--swap");
            args.push("--snet");
        }
        if matches.is_present("cpu") {
            args.push("--cpu")
        }
        if matches.is_present("load") {
            args.push("--load")
        }
        if matches.is_present("swap") {
            args.push("--swap")
        }
        if matches.is_present("net") {
            args.push("--snet")
        }
        if matches.is_present("Net") {
            args.push("--nets")
        }
        if matches.is_present("disk") {
            args.push("--disk")
        }

        let num_str = matches.value_of("intervals").unwrap_or("1");
        let n = num_str.parse::<u64>().unwrap();

        loop {
            thread::sleep(Duration::from_secs(n));
            // println!("{}",Local::now())
            monitor(args.clone());
        }
    }

    Ok(())
}

