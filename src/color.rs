// 文字字体 参数介绍：
// text->文本内容 
// status->文字颜色 
// background->背景颜色 
// underline->是否下划线 
// highlight->是否高亮
// http://www.cnblogs.com/frydsh/p/4139922.html
// https://stackoverflow.com/questions/33139248/i-cannot-print-color-escape-codes-to-the-terminal
// Rust 1.3.0 does not seem to support octal escape strings such as \033. Instead, you can use hexadecimal escape strings like \x1b.
pub fn colorize(text: &str, status: &str, background: &str, underline: bool, highlight: bool) -> String {
	let mut out_status = String::new();
	let mut out_bg = String::new();
	let mut underline_and_highlight = String::new();

	//可动态配置字体颜色 背景色 高亮
	// 显示：0(默认)、1(粗体/高亮)、22(非粗体)、4(单条下划线)、24(无下划线)、5(闪烁)、25(无闪烁)、7(反显、翻转前景色和背景色)、27(无反显)
	// 颜色：0(黑)、1(红)、2(绿)、 3(黄)、4(蓝)、5(洋红)、6(青)、7(白)
	//  前景色为30+颜色值，如31表示前景色为红色；背景色为40+颜色值，如41表示背景色为红色。
	if underline == true && highlight == true {
		underline_and_highlight = String::from(";1;4m")
	} else if underline != true && highlight == true {
		underline_and_highlight = String::from(";1m")
	} else if underline == true && highlight != true {
		underline_and_highlight = String::from(";4m")
	} else {
		underline_and_highlight = String::from(";22m")
	}
	

	match status {
		"black" => out_status = String::from("30"),
		"red" => out_status = String::from("31"),
		"green" => out_status = String::from("32"),
		"yellow" => out_status = String::from("33"),
		"blue" => out_status = String::from("34"),
		"purple" => out_status = String::from("35"),
		"dgreen" => out_status = String::from("36"),
		"white" => out_status = String::from("37"),
		_ => {}
	};

	match background {
		"black" => out_bg = String::from("40;"),
		"red" => out_bg = String::from("41;"),
		"green" => out_bg = String::from("42;"),
		"yellow" => out_bg = String::from("43;"),
		"blue" => out_bg = String::from("44;"),
		"purple" => out_bg = String::from("45;"),
		"dgreen" => out_bg = String::from("46;"),
		"white" => out_bg = String::from("47;"),
		_ => {}
	};

	format!("\x1b[{}{}{}{}\x1b[0m", out_bg ,out_status, underline_and_highlight,text)
}

pub fn color_test() {
    let rs: String = colorize("abcaaaaaaaaaaa", "red", "white", true, true);
    println!("{}", rs.to_string());
}