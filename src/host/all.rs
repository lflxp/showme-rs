use chrono::prelude::*;
use colored::*;
use psutil::*;

static mut COUNT: u32 = 0;
static REPLACE: &str = "oops";

#[derive(Debug)]
struct Title {}

impl Title {
	pub fn cpu() -> ColoredString {
		"----cpu-usage--- ".bright_blue()
	}

	pub fn cpucolumns() -> ColoredString {
		" usr sys idl iow|".bright_blue().underline()
	}

	pub fn time() -> ColoredString {
		"-------- ".bright_blue()
	}

	pub fn timecolumns() -> ColoredString {
		"  time  |".bright_blue().underline()
	}

	pub fn load() -> ColoredString {
		"-----load-avg---- ".bright_blue()
	}

	pub fn loadcolumns() -> ColoredString {
		"  1m    5m   15m |".bright_blue().underline()
	}

	pub fn swap() -> ColoredString {
		"---swap--- ".bright_blue()
	}

	pub fn swapcolumns() -> ColoredString {
		"   si   so|".bright_blue().underline()
	}

	pub fn net(detail: bool) -> ColoredString {
		if !detail {
			"----net(A)---- ".bright_blue()
		} else {
			"------------------------------net(Detail)----------------------------- ".bright_blue()
		}
	}

	pub fn netcolumns(detail: bool) -> ColoredString {
		if !detail {
			"   recv   send|".bright_blue().underline()
		} else {
			"   recv   send   psin   psot  errin  errot   dpin  dpout   ffin  ffout|".bright_blue().underline()
		}
	}

	pub fn disk() -> ColoredString {
		"------------------------io-usage---------------------- ".bright_blue()
	}

	pub fn diskcolumns() -> ColoredString {
		" readc writec    srkB    swkB queue  await svctm %util|".bright_blue().underline()
	}

	// pub fn com() -> ColoredString {
	// 	"-------QPS----------TPS------- ".bright_blue()
	// }

	// pub fn comcolumns() -> ColoredString {
	// 	"  ins   upd   del    sel   iud|".bright_blue().underline()
	// }

	// pub fn hit() -> ColoredString {
	// 	"----KeyBuffer------Index----Qcache---Innodb---(%) ".bright_blue()
	// }

	// pub fn hitcolumns() -> ColoredString {
	// 	"  read  write    cur  total lorhit readreq  inhit|".bright_blue().underline()
	// }

	// pub fn innodbrow() -> ColoredString {
	// 	"---innodb rows status--- ".bright_blue()
	// }

	// pub fn innodbrowcolumns() -> ColoredString {
	// 	"  ins   upd   del   read|".bright_blue().underline()
	// }

	// pub fn innodbpages() -> ColoredString {
	// 	"---innodb bp pages status-- ".bright_blue()
	// }

	// pub fn innodbpagescolumns() -> ColoredString {
	// 	"   data   free  dirty flush|".bright_blue().underline()
	// }

	// pub fn innodbdata() -> ColoredString {
	// 	"-----innodb data status----- ".bright_blue()
	// }

	// pub fn innodblog() -> ColoredString {
	// 	"--innodb log-- ".bright_blue()
	// }

	// pub fn innodblogcolmns() -> ColoredString {
	// 	"fsyncs written|".bright_blue().underline()
	// }

	// pub fn innodbstatus() -> ColoredString {
	// 	"  his --log(byte)--  read ---query--- ".bright_blue()
	// }

	// pub fn innodbstatuscolumns() -> ColoredString {
	// 	" list uflush  uckpt  view inside  que|".bright_blue().underline()
	// }

	// pub fn threads() -> ColoredString {
	// 	"----------threads--------- ".bright_blue()
	// }

	// pub fn threadscolumns() -> ColoredString {
	// 	" run  con  cre  cac   %hit|".bright_blue().underline()
	// }

	// pub fn bytes() -> ColoredString {
	// 	"-----bytes---- ".bright_blue()
	// }

	// pub fn bytescolumns() -> ColoredString {
	// 	"   recv   send|".bright_blue().underline()
	// }

	// pub fn semi() -> ColoredString {
	// 	"---avg_wait--tx_times--semi ".bright_blue()
	// }

	// pub fn semicolumns() -> ColoredString {
	// 	"  naw  txaw notx  yes   off|".bright_blue().underline()
	// }

	// pub fn slave() -> ColoredString {
	// 	"---------------SlaveStatus------------- ".bright_blue()
	// }

	// pub fn slavecolumns() -> ColoredString {
	// 	"ReadMLP ExecMLP   chkRE   SecBM|".bright_blue().underline()
	// }
}

fn parseRepeatSpace(info: String,lens: usize) -> String {
	let mut buf = info.clone();
	if info.len() > lens {
		buf = String::from(REPLACE)
	}
	format!("{}{}"," ".repeat(lens-buf.len()), buf)
}

// pub fn getswapio(beforein: u64,beforeout: u64) -> Result<String> {
// 	let swap_memory = memory::swap_memory().unwrap();
// 	let si = swap_memory.swapped_in().parse::<u64>() - beforein;
// 	let so = swap_memory.swapped_out().parse::<u64>() - beforeout;

// 	Ok(())
// }

fn get_time() -> String {
	format!("{}{}",
		format!("{}", Local::now().format("%H:%M:%S")).bright_yellow().to_string(),
		"|".green().to_string(),
	)
}

fn getdemo(info: &str) -> String {
	String::from(info)
}

use std::process::Command;
fn exec(cmd: &str) -> String {
	let output = Command::new("sh").arg("-c").arg(cmd).output().expect("命令异常");
	let x = String::from_utf8_lossy(&output.stdout);
	format!("{}", x)
}

fn gettitle(args: Vec<&str>) {
	let mut title = vec![Title::time().to_string()];
	let mut columns = vec![Title::timecolumns().to_string()];
	if args.contains(&"--load") {
		title.push(Title::load().to_string());
		columns.push(Title::loadcolumns().to_string());
	}

	if args.contains(&"--cpu") {
		title.push(Title::cpu().to_string());
		columns.push(Title::cpucolumns().to_string());
	}

	if args.contains(&"--swap") {
		title.push(Title::swap().to_string());
		columns.push(Title::swapcolumns().to_string());
	}

	if args.contains(&"--net") {
		title.push(Title::net(false).to_string());
		columns.push(Title::netcolumns(false).to_string());
	} else if args.contains(&"--nets") {
		title.push(Title::net(true).to_string());
		columns.push(Title::netcolumns(true).to_string());
	}

	if args.contains(&"--disk") {
		title.push(Title::disk().to_string());
		columns.push(Title::diskcolumns().to_string());
	}

	println!("{}", title.join(""));
	println!("{}", columns.join(""));
}

fn getload() -> String {
	let loadavg = host::loadavg().unwrap();
	let mut one: ColoredString = "".red();
	let mut two: ColoredString = "".red();
	let mut three: ColoredString = "".red();
	match loadavg.one as i32 {
		0..=10 => one = parseRepeatSpace(format!("{:.2}",loadavg.one), 5).green(),
		_ => one = parseRepeatSpace(format!("{:.2}",loadavg.one), 5).red()
	}

	match loadavg.five as i32 {
		0..=10 => two = parseRepeatSpace(format!("{:.2}",loadavg.five), 6).green(),
		_ => two = parseRepeatSpace(format!("{:.2}",loadavg.five), 6).red()
	}

	match loadavg.fifteen as i32 {
		0..=10 => three = parseRepeatSpace(format!("{:.2}",loadavg.fifteen), 6).green(),
		_ => three = parseRepeatSpace(format!("{:.2}",loadavg.fifteen), 6).red()
	}

	format!("{}{}{}{}",one,two,three,"|".green())
}

static mut SWAPIN: Bytes = 0;
static mut SWAPOUT: Bytes = 0;
fn getswap() -> String {
	let swap_memory = memory::swap_memory().unwrap();
	let mut sin = "".red();
	let mut sout = "".red();
	unsafe {
		let si: u64 = swap_memory.swapped_in() - SWAPIN;
		let so: u64 = swap_memory.swapped_out() - SWAPOUT;

		match si {
			0 => sin = parseRepeatSpace(si.to_string(),5).green(),
			_ => sin = parseRepeatSpace(si.to_string(),5).red()
		}

		match so {
			0 => sout = parseRepeatSpace(so.to_string(),5).green(),
			_ => sout = parseRepeatSpace(so.to_string(),5).red()
		}

		SWAPIN = swap_memory.swapped_in();
		SWAPOUT = swap_memory.swapped_out();
	}

	format!("{}{}{}",sin,sout,"|".green())
}

fn getcpu() -> String {
	let mut cpu_times_percent_collector = cpu::CpuTimesPercentCollector::new().unwrap();
	let cpu_times_percent_percpu = cpu_times_percent_collector
		.cpu_times_percent_percpu()
		.unwrap();
	let mut user:f32 = 0.0;
	let mut system:f32 = 0.0;
	let mut idle:f32 = 0.0;
	let mut busy:f32 = 0.0;
	for x in cpu_times_percent_percpu {
		user += x.user();
		system += x.system();
		idle += x.idle();
		busy += x.busy();
		// println!("user {} system {} idle {} busy {}",x.user(),x.system(),x.idle(),x.busy());
	}

	let mut user_str = "".red();
	let mut system_str = "".red();
	let mut idle_str = "".red();
	let mut busy_str = "".red();
	
	match user as u64 {
		0..=100 => user_str = parseRepeatSpace((user as u64/10 as u64).to_string(), 4).green(),
		_ => user_str = parseRepeatSpace((user as u64/10 as u64).to_string(), 4).red(), 
	}

	match system as u64 {
		0..=100 => system_str = parseRepeatSpace((system as u64/10 as u64).to_string(), 4).green(),
		_ => system_str = parseRepeatSpace((system as u64/10 as u64).to_string(), 4).red(), 
	}

	match idle as u64 {
		0..=300 => idle_str = parseRepeatSpace((system as u64/10 as u64).to_string(), 4).red(),
		_ => idle_str = parseRepeatSpace((system as u64/10 as u64).to_string(), 4).green(), 
	}

	match busy as u64 {
		0..=100 => busy_str = parseRepeatSpace((busy as u64/10 as u64).to_string(), 4).green(),
		_ => busy_str = parseRepeatSpace((busy as u64/10 as u64).to_string(), 4).red(), 
	}

	format!("{}{}{}{}{}",user_str,system_str,idle_str,busy_str,"|".green())
	// format!("{} {} {} {}",user,system,idle,busy)
}

fn getdata(args: Vec<&str>) {
	let mut rs = vec![get_time()];
	if args.contains(&"--load") {
		rs.push(getload());
	}

	if args.contains(&"--cpu") {
		rs.push(getcpu());
	}

	if args.contains(&"--swap") {
		rs.push(getswap());
	}

	if args.contains(&"--net") {
		rs.push(getdemo("net"));
	} else if args.contains(&"--nets") {
		rs.push(getdemo("Net"));
	}

	if args.contains(&"--disk") {
		rs.push(getdemo("disk"));
	}

	println!("{}", rs.join(""));
}

// print basic info 
fn show_hardware() {
	let uptime = host::uptime().unwrap().as_secs();
	println!("{}: {} days", 
		"     Uptime       ".bright_white().on_red().underline(),
		format!("{:.2}",uptime/24/60/60).bright_green().bold());
	let hostname = exec("hostname");
	println!("{}: {}", 
		"     Hostname     ".bright_white().on_red().underline(),
		hostname.trim().bright_green().bold());
	let loadavg = host::loadavg().unwrap();
	println!("{}: {}", 
		"     Load         ".bright_white().on_red().underline(),
		format!("{:.2} {:.2} {:.2}",loadavg.one,loadavg.five,loadavg.fifteen).bright_green().bold()
	);

	let disk_usage = disk::disk_usage("/").unwrap();
	println!("{}: {}", 
		"     DISK         ".bright_white().on_red().underline(),
		format!("Total {:.2} Used {:.2} Free {:.2} Percent {:.2}",disk_usage.total()/1024/1024/1024,disk_usage.used()/1024/1024/1024,disk_usage.free()/1024/1024/1024,disk_usage.percent()).bright_green().bold()
	);

	let virtual_memory = memory::virtual_memory().unwrap();
	println!("{}: {}", 
		"     MEM          ".bright_white().on_red().underline(),
		format!("Total {:.2} Available {:.2} Used {:.2} Free {:.2} Percent {:.2}",virtual_memory.total()/1024/1024/1024,virtual_memory.available()/1024/1024/1024,virtual_memory.used()/1024/1024/1024,virtual_memory.free()/1024/1024/1024,disk_usage.percent()).bright_green().bold()
	);
	let swap_memory = memory::swap_memory().unwrap();
	println!("{}: {}", 
		"     SWAP         ".bright_white().on_red().underline(),
		format!("Total {:.2} Used {:.2} Free {:.2} Percent {:.2}",swap_memory.total()/1024/1024/1024,swap_memory.used()/1024/1024/1024,swap_memory.free()/1024/1024/1024,swap_memory.percent()).bright_green().bold()
	);

	println!("{}",parseRepeatSpace(String::from("991asdaz"), 6))
}

// 打印
pub fn monitor(args: Vec<&str>) {
	unsafe {
		if COUNT == 0 {
			show_hardware();
		}

		if COUNT%20 == 0 {
			gettitle(args);
		} else {
			getdata(args);	
		}
		COUNT += 1;
	}
}