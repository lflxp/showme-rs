// mod c2;
// use c2::c2;

use log::{info};
use log4rs;

mod colorrun;
use colorrun::colorrun;

// mod color;
// use color::color_test;

// mod clapsed;
// use clapsed::clap_init;
use psutil::*;
// use colored::*;

mod host;
use host::*;

use chrono::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("Starting Scan!!!");

    colorrun();
    // color_test();

    // clap_init();

    // let partitions = disk::partitions_physical().unwrap();
    // info!("Partitions {:?}", partitions);

    // dbg!(partitions);

    // let mut cpu_percent_collector = cpu::CpuPercentCollector::new().unwrap();
	// let cpu_percents_percpu = cpu_percent_collector.cpu_percent_percpu().unwrap();

    // dbg!(cpu_percents_percpu);

    // let disk_usage = disk::disk_usage("/").unwrap();

	// let uptime = host::uptime().unwrap();
	// let boot_time = host::boot_time().unwrap();
	// let loadavg = host::loadavg().unwrap();

    // dbg!(disk_usage);
    // dbg!(uptime);
    // {
    //     info!("11111 {:?}", boot_time.elapsed());
    // }
    // dbg!(boot_time);
    // {

    //     let x = format!("{}", loadavg.one).red().bold();
    //     info!("{}",x.to_string());
    // }
    // dbg!(loadavg);

    // let virtual_memory = memory::virtual_memory().unwrap();
	// let swap_memory = memory::swap_memory().unwrap();

    // dbg!(virtual_memory);
    // dbg!(swap_memory);

    // let mut cpu_percent_collector = cpu::CpuPercentCollector::new().unwrap();
	// let mut cpu_times_percent_collector = cpu::CpuTimesPercentCollector::new().unwrap();

    // let cpu_percents_percpu = cpu_percent_collector.cpu_percent_percpu().unwrap();
	// let cpu_times_percpu = cpu::cpu_times_percpu().unwrap();
	// let cpu_times_percent_percpu = cpu_times_percent_collector
	// 	.cpu_times_percent_percpu()
	// 	.unwrap();

    // // let x = gethostname().red();
    // // info!("{}",x.to_string());

    // dbg!(cpu_percents_percpu);
	// dbg!(cpu_times_percpu);
	// dbg!(cpu_times_percent_percpu);


    loop {
        thread::sleep(Duration::from_secs(1));
        // println!("{}",Local::now())
        monitor()
    }
}