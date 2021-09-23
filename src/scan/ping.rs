use stopwatch::{Stopwatch};
use oping::{Ping, PingResult};
use log::{error, info, warn};

// static mut IS_LOOPING: bool = true;
static mut SUCCESS: u32 = 0;
static mut FAILURE: u32 = 0;
static mut TIME: f64 = 0.0;

pub async fn pingmethod(addr: Vec<String>) -> PingResult<Vec<String>> {
    let mut list: Vec<String> = Vec::new();

    // create ICMP packet using external library oping
    // while searching for external rust libraries, I realized
    // many of them are not being updated anymore or was abandoned.
    let mut ping = Ping::new();
    
    // max wait time is 5 seconds
    match ping.set_timeout(0.1) {
        Ok(_) => {},
        Err(e) => {
            error!("{}", e);
        }
    };

    // println!("ip is |{}|", addr);
    for host in addr {
        match ping.add_host(&host) {
            Ok(_) => {},
            Err(e) => error!("{:?}",e)
        }
    }       

    // send ICMP packet
    let responses = ping.send()?;

    unsafe {
        // check response and update result
        for response in responses {
            if response.dropped > 0 {
                warn!("No response from host {} (loss)", response.address);
                FAILURE += 1;
            } else {
                // display success result
                info!("Response from host {} (address {}): latency {} ms", response.hostname, response.address, response.latency_ms);
                SUCCESS += 1;
                TIME += response.latency_ms;
                list.push(response.address.to_string());
            }

            // if response.dropped <= 0 {
            //     println!("Response from host {} (address {}): latency {} ms", response.hostname, response.address, response.latency_ms);
            // }
        }
    }

    Ok(list)
}

pub async fn print_result(tt: Stopwatch) {
    println!("--- Ping result ---");
    unsafe {
        info!("TOTAL  : {} packets", SUCCESS + FAILURE);
        info!("SUCCESS: {}", SUCCESS);
        info!("FAILURE: {}", FAILURE);
        // safe casting using keyword as
        info!("TIME   : {:.3} ms", TIME / (SUCCESS + FAILURE) as f64);
        info!("Times: {:.3} ms", tt.elapsed_ms() as f64)
    }
}