use std::{process::{Command, Stdio, Child}, io::{stdin, stdout, Write}, path::Path, env};

pub fn execShell(cmd: String){
    stdout().flush();

    // let input = String::from("kubectl edit po nginx");
    // stdin().read_line(&mut input).unwrap();

    // everything after the first whitespace character 
    //     is interpreted as args to the command
    // 第一个空白符之后的所有内容都视为命令的参数
    let mut parts = cmd.trim().split_whitespace();
    let command = parts.next().unwrap();
    let args = parts;

    let mut child = Command::new(command)
        .args(args)
        .spawn();

    // 优雅地处理非正常输入
    match child {
        Ok(mut child) => { child.wait(); },
        Err(e) => eprintln!("{}", e),
    };
}