// use std::process::Command;

// pub fn gethostname() -> String {
// 	let output = Command::new("sh").arg("-c").arg("hostname").output().expect("命令异常");
// 	let x = String::from_utf8_lossy(&output.stdout);
// 	format!("{}", x)
// }