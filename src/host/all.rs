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
			// "------------------------------net(Detail)----------------------------- ".bright_blue()
			"-----------------------net(Detail)---------------------- ".bright_blue()
		}
	}

	pub fn netcolumns(detail: bool) -> ColoredString {
		if !detail {
			"   recv   send|".bright_blue().underline()
		} else {
			// "   recv   send   psin   psot  errin  errot   dpin  dpout   ffin  ffout|".bright_blue().underline()
			"   recv   send   psin   psot  errin  errot   dpin  dpout|".bright_blue().underline()
		}
	}

	pub fn disk() -> ColoredString {
		// "------------------------io-usage---------------------- ".bright_blue()
		"-----------io-usage---------- ".bright_blue()
	}

	pub fn diskcolumns() -> ColoredString {
		// " readc writec    srkB    swkB queue  await svctm %util|".bright_blue().underline()
		" readc writec    srkB    swkB|".bright_blue().underline()
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

fn parse_repeat_space(info: String,lens: usize) -> String {
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

	if args.contains(&"--snet") {
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
	let one: ColoredString;
	let two: ColoredString;
	let three: ColoredString;
	match loadavg.one as i32 {
		0..=10 => one = parse_repeat_space(format!("{:.2}",loadavg.one), 5).green(),
		_ => one = parse_repeat_space(format!("{:.2}",loadavg.one), 5).red()
	}

	match loadavg.five as i32 {
		0..=10 => two = parse_repeat_space(format!("{:.2}",loadavg.five), 6).green(),
		_ => two = parse_repeat_space(format!("{:.2}",loadavg.five), 6).red()
	}

	match loadavg.fifteen as i32 {
		0..=10 => three = parse_repeat_space(format!("{:.2}",loadavg.fifteen), 6).green(),
		_ => three = parse_repeat_space(format!("{:.2}",loadavg.fifteen), 6).red()
	}

	format!("{}{}{}{}",one,two,three,"|".green())
}

static mut SWAPIN: Bytes = 0;
static mut SWAPOUT: Bytes = 0;
fn getswap() -> String {
	let swap_memory = memory::swap_memory().unwrap();
	let sin: ColoredString;
	let sout: ColoredString;
	unsafe {
		let si: u64 = swap_memory.swapped_in() - SWAPIN;
		let so: u64 = swap_memory.swapped_out() - SWAPOUT;

		match si {
			0 => sin = parse_repeat_space(si.to_string(),5).green(),
			_ => sin = parse_repeat_space(si.to_string(),5).red()
		}

		match so {
			0 => sout = parse_repeat_space(so.to_string(),5).green(),
			_ => sout = parse_repeat_space(so.to_string(),5).red()
		}

		SWAPIN = swap_memory.swapped_in();
		SWAPOUT = swap_memory.swapped_out();
	}

	format!("{}{}{}",sin,sout,"|".green())
}

use std::fs;

static STAT: &str = "/proc/stat";
static mut USER: u64 = 0;
static mut NICE: u64 = 0;
static mut SYSTEM: u64 = 0;
static mut IDLE: u64 = 0;
static mut IOWAIT: u64 = 0;
static mut IRQ: u64 = 0;
static mut SOFTIRQ: u64 = 0;
static mut STEAL: u64 = 0;
// cpu  966383 1303 426944 72125242 14193 0 58033 0 0 0
fn getcpu() -> String {
	let file = fs::read_to_string(STAT).unwrap();
	let contents: Vec<_> = file
		.lines()
		.take_while(|line| line.starts_with("cpu "))
		.collect();
	// println!("cpu {:?}", contents[0].split_whitespace());
	let mut user: u64 = 0;
	let mut nice: u64 = 0;
	let mut system: u64 = 0;
	let mut idle: u64 = 0;
	let mut iowait: u64 = 0;
	let mut irq: u64 = 0;
	let mut softirq: u64 = 0;
	let mut steal: u64 = 0;
	for (index, x) in contents[0].split_whitespace().into_iter().enumerate() {
		// println!("index {} {}",index,x);
		match index {
			1 => user = x.parse::<u64>().unwrap(),
			2 => nice = x.parse::<u64>().unwrap(),
			3 => system = x.parse::<u64>().unwrap(),
			4 => idle = x.parse::<u64>().unwrap(),
			5 => iowait = x.parse::<u64>().unwrap(),
			6 => irq = x.parse::<u64>().unwrap(),
			7 => softirq = x.parse::<u64>().unwrap(),
			8 => steal = x.parse::<u64>().unwrap(),
			_ => {},
		}
	}

	unsafe {
		let cpu_total1: u64 = USER + NICE + SYSTEM + IDLE + IOWAIT + IRQ + SOFTIRQ;
		let cpu_total2: u64 = user + nice + system + idle + iowait + irq + softirq;
		
		let user_c:u64 = (user-USER) * 100 / (cpu_total2 - cpu_total1);
		let system_c:u64 = (system-SYSTEM) * 100 / (cpu_total2 - cpu_total1);
		let idle_c:u64 = (idle-IDLE) * 100 / (cpu_total2 - cpu_total1);
		let iow_c:u64 = (iowait-IOWAIT) * 100 / (cpu_total2 - cpu_total1);
		

		let user_str: ColoredString;
		let system_str: ColoredString;
		let idle_str: ColoredString;
		let iowait_str: ColoredString;
		
		match user_c {
			0..=10 => user_str = parse_repeat_space(user_c.to_string(), 4).green(),
			11..=30 => user_str = parse_repeat_space(user_c.to_string(), 4).yellow(),
			_ => user_str = parse_repeat_space(user_c.to_string(), 4).red(), 
		}

		match system_c {
			0..=10 => system_str = parse_repeat_space(system_c.to_string(), 4).green(),
			11..=30 => system_str = parse_repeat_space(system_c.to_string(), 4).yellow(),
			_ => system_str = parse_repeat_space(system_c.to_string(), 4).red(), 
		}

		match idle_c {
			0..=30 => idle_str = parse_repeat_space(idle_c.to_string(), 4).red(),
			31..=80 => idle_str = parse_repeat_space(idle_c.to_string(), 4).yellow(),
			_ => idle_str = parse_repeat_space(idle_c.to_string(), 4).green(), 
		}

		match iow_c as u64 {
			0..=10 => iowait_str = parse_repeat_space(iow_c.to_string(), 4).green(),
			11..=30 => iowait_str = parse_repeat_space(iow_c.to_string(), 4).yellow(),
			_ => iowait_str = parse_repeat_space(iow_c.to_string(), 4).red(), 
		}

		USER = user;
		NICE = nice;
		SYSTEM = system;
		IDLE= idle;
		IOWAIT = iowait;
		IRQ = irq;
		SOFTIRQ = softirq;
		STEAL = steal;

		format!("{}{}{}{}{}",user_str,system_str,idle_str,iowait_str,"|".green())
		// format!("{} {} {} {}",user,system,idle,busy)
	}
}

static mut BS: u64 = 0;
static mut BR: u64 = 0;
static mut PS: u64 = 0;
static mut PR: u64 = 0;
static mut ERRIN: u64 = 0;
static mut ERROUT: u64 = 0;
static mut DROPIN: u64 = 0;
static mut DROPOUT: u64 = 0;

#[warn(unused_mut)]
fn getnetio(detail: bool) -> String {
	let mut net_io_counters_collector = network::NetIoCountersCollector::default();
	let net_io_counters = net_io_counters_collector.net_io_counters().unwrap();

	let bytes_send = net_io_counters.bytes_sent();
	let bytes_recv = net_io_counters.bytes_recv();
	let packets_sent = net_io_counters.packets_sent();
	let packets_recv = net_io_counters.packets_recv();
	let err_in = net_io_counters.err_in();
	let err_out = net_io_counters.err_out();
	let drop_in = net_io_counters.drop_in();
	let drop_out = net_io_counters.drop_out();

	// println!("{} {} {} {}",bytes_send,bytes_recv,packets_sent,packets_recv);

	let result: String;
	unsafe {
		if !detail {
			let netin = bytes_recv - BR;
			let netout = bytes_send - BS;

			let netin_str: ColoredString;
			let netout_str: ColoredString;
			match netin/1024 {
				0 => netin_str = parse_repeat_space(format!("{:.0}",netin), 7).white(),
				1..=1000 => netin_str = parse_repeat_space(format!("{:.0}k",netin/1024), 7).yellow(),
				_ => netin_str = parse_repeat_space(format!("{:.1}m",netin/1024/1024), 7).red(),
			}

			match netout/1024 {
				0 => netout_str = parse_repeat_space(format!("{:.0}",netout), 7).white(),
				1..=1000 => netout_str = parse_repeat_space(format!("{:.0}k",netout/1024), 7).yellow(),
				_ => netout_str = parse_repeat_space(format!("{:.1}m",netout/1024/1024), 7).red(),
			}

			if BR == 0 && BS == 0 {
				result = format!("      0      0{}","|".green());
			} else {
				result = format!("{}{}{}",netin_str,netout_str,"|".green());
			}
			BS = bytes_send;
			BR = bytes_recv;			
		} else {
			let netin = bytes_recv - BR;
			let netout = bytes_send - BS;
			let pout = packets_sent -PS;
			let pin = packets_recv -PR;
			let errin = err_in - ERRIN;
			let errout = err_out - ERROUT;
			let dropin = drop_in -DROPIN;
			let dropout = drop_out -DROPOUT;


			let netin_str: ColoredString;
			let netout_str: ColoredString;
			let pout_str: ColoredString;
			let pin_str: ColoredString;
			let errin_str: ColoredString;
			let errout_str: ColoredString;
			let dropin_str: ColoredString;
			let dropout_str: ColoredString;

			match netin/1024 {
				0 => netin_str = parse_repeat_space(format!("{:.0}",netin), 7).white(),
				1..=1000 => netin_str = parse_repeat_space(format!("{:.0}k",netin/1024), 7).yellow(),
				_ => netin_str = parse_repeat_space(format!("{:.1}m",netin/1024/1024), 7).red(),
			}

			match netout/1024 {
				0 => netout_str = parse_repeat_space(format!("{:.0}",netout), 7).white(),
				1..=1000 => netout_str = parse_repeat_space(format!("{:.0}k",netout/1024), 7).yellow(),
				_ => netout_str = parse_repeat_space(format!("{:.1}m",netout/1024/1024), 7).red(),
			}

			match pout/1000 {
				0 => pout_str = parse_repeat_space(format!("{:.0}",pout), 7).white(),
				1..=1000 => pout_str = parse_repeat_space(format!("{:.0}k",pout/1000), 7).white(),
				_ => pout_str = parse_repeat_space(format!("{:.0}m",pout), 7).white(),
			}

			match pin/1000 {
				0 => pin_str = parse_repeat_space(format!("{:.0}",pin), 7).white(),
				1..=1000 => pin_str = parse_repeat_space(format!("{:.0}k",pin/1000), 7).white(),
				_ => pin_str = parse_repeat_space(format!("{:.0}m",pin), 7).white(),
			}

			match errin/1000 {
				0 => errin_str = parse_repeat_space(format!("{:.0}",errin), 7).white(),
				1..=1000 => errin_str = parse_repeat_space(format!("{:.0}k",errin/1000), 7).white(),
				_ => errin_str = parse_repeat_space(format!("{:.0}m",errin), 7).white(),
			}

			match errout/1000 {
				0 => errout_str = parse_repeat_space(format!("{:.0}",errout), 7).white(),
				1..=1000 => errout_str = parse_repeat_space(format!("{:.0}k",errout/1000), 7).white(),
				_ => errout_str = parse_repeat_space(format!("{:.0}m",errout), 7).white(),
			}

			match dropin/1000 {
				0 => dropin_str = parse_repeat_space(format!("{:.0}",dropin), 7).white(),
				1..=1000 => dropin_str = parse_repeat_space(format!("{:.0}k",dropin/1000), 7).white(),
				_ => dropin_str = parse_repeat_space(format!("{:.0}m",dropin), 7).white(),
			}

			match dropout/1000 {
				0 => dropout_str = parse_repeat_space(format!("{:.0}",dropout), 7).white(),
				1..=1000 => dropout_str = parse_repeat_space(format!("{:.0}k",dropout/1000), 7).white(),
				_ => dropout_str = parse_repeat_space(format!("{:.0}m",dropout), 7).white(),
			}

			if BR == 0 && BS == 0 {
				// result = format!("      0      0{}","|".green());
				result = format!("      0      0{}{}{}{}{}{}{}",pin_str,pout_str,errin_str,errout_str,dropin_str,dropout_str,"|".green());
			} else {
				result = format!("{}{}{}{}{}{}{}{}{}",netin_str,netout_str,pin_str,pout_str,errin_str,errout_str,dropin_str,dropout_str,"|".green());
			}
			
			BS = bytes_send;
			BR = bytes_recv;
			PS = packets_sent;
			PR = packets_recv;
			ERRIN = err_in;
			ERROUT = err_out;
			DROPIN = drop_in;
			DROPOUT = drop_out;
		}
		result
	}
}

// static mut READCOUNT: u64 = 0;
// static mut WRITECOUNT: u64 = 0;
// static mut READBYTES: u64 = 0;
// static mut WRITEBYTES: u64 = 0;

// fn getdisk() -> String {
// 	let mut disk_io_counters_collector = disk::DiskIoCountersCollector::default();
// 	let disk_io_counters_per_partition = disk_io_counters_collector
// 		.disk_io_counters_per_partition()
// 		.unwrap();

// 	let mut read_count: u64 = 0;
// 	let mut write_count: u64;
// 	let mut read_bytes: u64;
// 	let mut write_bytes: u64;
// 	for x in disk_io_counters_per_partition {
// 		let x1 = match x.read_count() {
// 			Ok(tmp) => tmp,
// 			Err(_) => {}
// 		}
// 		read_count += x.read_count();
// 	}
// 	String::from("")
// }

fn getdata(args: Vec<&str>) {
	let mut rs = vec![get_time()];
	if args.contains(&"--lazy") {
		rs.push(getload());
		rs.push(getcpu());
		rs.push(getswap());
		rs.push(getnetio(false));
	}
	if args.contains(&"--load") {
		rs.push(getload());
	}

	if args.contains(&"--cpu") {
		rs.push(getcpu());
	}

	if args.contains(&"--swap") {
		rs.push(getswap());
	}

	if args.contains(&"--snet") {
		rs.push(getnetio(false));
	} else if args.contains(&"--nets") {
		rs.push(getnetio(true));
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

	println!("{}",parse_repeat_space(String::from("991asdaz"), 6))
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