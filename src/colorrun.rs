use log::{info};

extern crate colored; // not needed in Rust 2018

use colored::*;

// 这个函数仅当操作系统是 Linux 的时候才会编译
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!")
}

// 而这个函数仅当操作系统**不是** Linux 时才会编译
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!")
}

pub fn colorrun() {

    are_you_on_linux();
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }

    info!("{} {}!","it".green().bold(),"works".blue().bold().on_blue());

    println!("{}", "this is blue".blue());
    println!("{}", ("this is red".red()));
    println!("{}", ("this is red on blue".red().on_blue()));
    println!("{}", ("this is also red on blue".on_blue().red()));
    println!("{}", ("you can use truecolor values too!".truecolor(0, 255, 136)));
    println!("{}", ("background truecolor also works :)".on_truecolor(135, 28, 167)));
    println!("{}", ("bright colors are welcome as well".on_bright_blue().bright_red()));
    println!("{}", ("you can also make bold comments".bold()));
    println!("{} {} {}", "or use".cyan(), "any".italic().yellow(), "string type".cyan());
    println!("{}", ("or change advice. This is red".yellow().blue().red()));
    println!("{}", ("or clear things up. This is default color and style".red().bold().clear()));
    println!("{}", ("purple and magenta are the same".purple().magenta()));
    println!("{}", ("and so are normal and clear".normal().clear()));
    println!("{}", ("you can specify color by string".color("blue").on_color("red")));
    String::from("this also works!").green().bold();
    format!("{:30}", "format works as expected. This will be padded".blue());
    format!("{:.3}", "and this will be green but truncated to 3 chars".green());
}
