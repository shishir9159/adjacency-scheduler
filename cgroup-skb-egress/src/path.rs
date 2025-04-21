use ping::{ping, PingResult};
use std::net::IpAddr;
use std::str::FromStr;
use std::time::Duration;

fn nodeping() -> bool {
    let address = "cgroup-skb-egress.default.cluster.local";
    let ip = IpAddr::from_str(address).expect("couldn't resolve");
    let count = 100;
    let timeout = Duration::from_secs(1);

    let mut total_time = 0f64;
    let mut success_count = 0;

    println!("Pinging {} {} times...", address, count);

    for i in 1..=count {
        match ping(ip, Some(timeout), None, None) {
            Ok(PingResult::Pong(rtt)) => {
                println!("Ping {}: {:.2?} ms", i, rtt.as_secs_f64() * 1000.0);
                total_time += rtt.as_secs_f64() * 1000.0;
                success_count += 1;
            }
            Ok(PingResult::Timeout) => println!("Ping {}: Timed out", i),
            Err(e) => println!("Ping {}: Error - {}", i, e),
        }
    }

    if success_count > 0 {
        let avg = total_time / success_count as f64;
        println!(
            "\nAverage latency over {} successful pings: {:.2} ms",
            success_count, avg
        );
    } else {
        println!("\nAll pings failed.");
    }

    false
}
