use chrono::prelude::*;
use colored::*;

static mut COUNT: u32 = 0;

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

	pub fn com() -> ColoredString {
		"-------QPS----------TPS------- ".bright_blue()
	}

	pub fn comcolumns() -> ColoredString {
		"  ins   upd   del    sel   iud|".bright_blue().underline()
	}

	pub fn hit() -> ColoredString {
		"----KeyBuffer------Index----Qcache---Innodb---(%) ".bright_blue()
	}

	pub fn hitcolumns() -> ColoredString {
		"  read  write    cur  total lorhit readreq  inhit|".bright_blue().underline()
	}

	pub fn innodbrow() -> ColoredString {
		"---innodb rows status--- ".bright_blue()
	}

	pub fn innodbrowcolumns() -> ColoredString {
		"  ins   upd   del   read|".bright_blue().underline()
	}

	pub fn innodbpages() -> ColoredString {
		"---innodb bp pages status-- ".bright_blue()
	}

	pub fn innodbpagescolumns() -> ColoredString {
		"   data   free  dirty flush|".bright_blue().underline()
	}

	pub fn innodbdata() -> ColoredString {
		"-----innodb data status----- ".bright_blue()
	}

	pub fn innodblog() -> ColoredString {
		"--innodb log-- ".bright_blue()
	}

	pub fn innodblogcolmns() -> ColoredString {
		"fsyncs written|".bright_blue().underline()
	}

	pub fn innodbstatus() -> ColoredString {
		"  his --log(byte)--  read ---query--- ".bright_blue()
	}

	pub fn innodbstatuscolumns() -> ColoredString {
		" list uflush  uckpt  view inside  que|".bright_blue().underline()
	}

	pub fn threads() -> ColoredString {
		"----------threads--------- ".bright_blue()
	}

	pub fn threadscolumns() -> ColoredString {
		" run  con  cre  cac   %hit|".bright_blue().underline()
	}

	pub fn bytes() -> ColoredString {
		"-----bytes---- ".bright_blue()
	}

	pub fn bytescolumns() -> ColoredString {
		"   recv   send|".bright_blue().underline()
	}

	pub fn semi() -> ColoredString {
		"---avg_wait--tx_times--semi ".bright_blue()
	}

	pub fn semicolumns() -> ColoredString {
		"  naw  txaw notx  yes   off|".bright_blue().underline()
	}

	pub fn slave() -> ColoredString {
		"---------------SlaveStatus------------- ".bright_blue()
	}

	pub fn slavecolumns() -> ColoredString {
		"ReadMLP ExecMLP   chkRE   SecBM|".bright_blue().underline()
	}
}

fn get_time() -> String {
	format!("{}{}",
		format!("{}", Local::now().format("%H:%M:%S")).bright_yellow().to_string(),
		"|".green().to_string(),
	)
}

fn getdemo(info: &str) -> String {
	String::from(info)
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

fn getdata(args: Vec<&str>) {
	let mut rs = vec![get_time()];
	if args.contains(&"--load") {
		rs.push(getdemo("load"));
	}

	if args.contains(&"--cpu") {
		rs.push(getdemo("cpu"));
	}

	if args.contains(&"--swap") {
		rs.push(getdemo("swap"));
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

// // 打印title
// fn title() -> String {

// 	"ok"
// }

// 打印
pub fn monitor(args: Vec<&str>) {
	unsafe {
		if COUNT%20 == 0 {
			gettitle(args);
		} else {
			getdata(args);	
		}
		COUNT += 1;
	}
}