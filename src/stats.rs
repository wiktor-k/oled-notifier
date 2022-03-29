use systemstat::data::IpAddr;
use systemstat::data::LoadAverage;
use systemstat::data::Memory;
use systemstat::data::Network;

use std::collections::BTreeMap;
use std::io::Result;
use std::time::Duration;

pub fn format_load_average(avg: Result<LoadAverage>) -> String {
    match avg {
        Ok(loadavg) => format!(
            "C: {:.0}% {:.0}% {:.0}%",
            loadavg.one * 100.0,
            loadavg.five * 100.0,
            loadavg.fifteen * 100.0
        ),
        Err(x) => format!("CPU: error: {}", x),
    }
}

pub fn format_memory(mem: Result<Memory>) -> String {
    match mem {
        Ok(mem) => format!(
            "M: {}/{} MB",
            mem.free.as_u64() / (1024 * 1024),
            mem.total.as_u64() / (1024 * 1024),
        ),
        Err(x) => format!("\nMem: error: {}", x),
    }
}

pub fn format_networks(net: Result<BTreeMap<String, Network>>) -> String {
    match net {
        Ok(netifs) => {
            if let Some(netif) = netifs.values().next() {
                if let Some(addr) = netif.addrs.get(0) {
                    match addr.addr {
                        IpAddr::V4(v4) => format!("N: {}", v4),
                        _ => "Net: unknown".to_string(),
                    }
                } else {
                    "Net: no addr".to_string()
                }
            } else {
                "Net: no found".to_string()
            }
        }
        Err(x) => format!("Net: error: {}", x),
    }
}

pub fn format_uptime(up: Result<Duration>) -> String {
    match up {
        Ok(uptime) => format!("U: {}", crate::duration::FormattedDuration::new(uptime)),
        Err(x) => format!("Up: error: {}", x),
    }
}
