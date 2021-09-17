use chrono::prelude::*;
use colored::*;

static mut COUNT: u32 = 0;

#[derive(Debug)]
struct Title {}

impl Title {
	pub fn cputitle() -> ColoredString {
		"----cpu-usage--- ".bright_blue()
	}

	pub fn cpucolumn() -> ColoredString {
		" usr sys idl iow|".bright_blue()
	}

	pub fn time() -> ColoredString {
		"-------- ".bright_blue()
	}

	pub fn timecolumns() -> ColoredString {
		"  time  |".bright_blue()
	}

	pub fn load() -> ColoredString {
		"-----load-avg---- ".bright_blue()
	}

	pub fn loadcolumns() -> ColoredString {
		"  1m    5m   15m |".bright_blue()
	}

	pub fn swap() -> ColoredString {
		"---swap--- ".bright_blue()
	}

	pub fn swapcolumns() -> ColoredString {
		"   si   so|".bright_blue()
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
			"   recv   send|".bright_blue()
		} else {
			"   recv   send   psin   psot  errin  errot   dpin  dpout   ffin  ffout|".bright_blue()
		}
	}

	pub fn disk() -> ColoredString {
		"------------------------io-usage---------------------- ".bright_blue()
	}

	pub fn diskcolumns() -> ColoredString {
		" readc writec    srkB    swkB queue  await svctm %util|".bright_blue()
	}

	pub fn com() -> ColoredString {
		"-------QPS----------TPS------- ".bright_blue()
	}

	pub fn comcolumns() -> ColoredString {
		"  ins   upd   del    sel   iud|".bright_blue()
	}

	pub fn hit() -> ColoredString {
		"----KeyBuffer------Index----Qcache---Innodb---(%) ".bright_blue()
	}

	pub fn hitcolumns() -> ColoredString {
		"  read  write    cur  total lorhit readreq  inhit|".bright_blue()
	}

	pub fn innodbrow() -> ColoredString {
		"---innodb rows status--- ".bright_blue()
	}

	pub fn innodbrowcolumns() -> ColoredString {
		"  ins   upd   del   read|".bright_blue()
	}

	pub fn innodbpages() -> ColoredString {
		"---innodb bp pages status-- ".bright_blue()
	}

	pub fn innodbpagescolumns() -> ColoredString {
		"   data   free  dirty flush|".bright_blue()
	}

	pub fn innodbdata() -> ColoredString {
		"-----innodb data status----- ".bright_blue()
	}

	pub fn innodblog() -> ColoredString {
		"--innodb log-- ".bright_blue()
	}

	pub fn innodblogcolmns() -> ColoredString {
		"fsyncs written|".bright_blue()
	}

	pub fn innodbstatus() -> ColoredString {
		"  his --log(byte)--  read ---query--- ".bright_blue()
	}

	pub fn innodbstatuscolumns() -> ColoredString {
		" list uflush  uckpt  view inside  que|".bright_blue()
	}

	pub fn threads() -> ColoredString {
		"----------threads--------- ".bright_blue()
	}

	pub fn threadscolumns() -> ColoredString {
		" run  con  cre  cac   %hit|".bright_blue()
	}

	pub fn bytes() -> ColoredString {
		"-----bytes---- ".bright_blue()
	}

	pub fn bytescolumns() -> ColoredString {
		"   recv   send|".bright_blue()
	}

	pub fn semi() -> ColoredString {
		"---avg_wait--tx_times--semi ".bright_blue()
	}

	pub fn semicolumns() -> ColoredString {
		"  naw  txaw notx  yes   off|".bright_blue()
	}

	pub fn slave() -> ColoredString {
		"---------------SlaveStatus------------- ".bright_blue()
	}

	pub fn slavecolumns() -> ColoredString {
		"ReadMLP ExecMLP   chkRE   SecBM|".bright_blue()
	}
}

fn gettitle() {
	let mut title = vec![Title::time()];
	let mut columns = vec![Title::timecolumns()];
	unsafe {
		println!("{}", title.join(""));
	}
}

// // 打印title
// fn title() -> String {

// 	"ok"
// }

// 打印开头
pub fn monitor() {
	unsafe {
		if COUNT%10 == 0 {
			println!("{}",Title::cputitle());
			println!("{}",Title::cpucolumn());
		} else {
			let now = Local::now().format("%H:%M:%S");
			println!("{}", now)
		}
		COUNT += 1;
	}
}