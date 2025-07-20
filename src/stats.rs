use systemstat::DelayedMeasurement;
use systemstat::data::IpAddr;
use systemstat::data::Memory;
use systemstat::data::Network;

use std::collections::BTreeMap;
use std::io::Result;
use std::time::Duration;

pub fn format_cpu_load(avg: Result<DelayedMeasurement<Vec<systemstat::data::CPULoad>>>) -> String {
    match avg {
        Ok(measurement) => {
            std::thread::sleep(std::time::Duration::from_secs(2));
            match measurement.done() {
                Ok(load) => load
                    .into_iter()
                    .map(|load| format!("{}%", (load.user * 100.0).round()))
                    .collect::<Vec<_>>()
                    .join(" "),
                Err(x) => format!("CPU: finish error: {x}"),
            }
        }
        Err(x) => format!("CPU: start error: {x}"),
    }
}

pub fn format_memory(mem: Result<Memory>) -> String {
    match mem {
        Ok(mem) => format!(
            "{} / {} MB",
            mem.free.as_u64() / (1024 * 1024),
            mem.total.as_u64() / (1024 * 1024),
        ),
        Err(x) => format!("\nMem: error: {x}"),
    }
}

pub fn format_networks(net: Result<BTreeMap<String, Network>>) -> String {
    match net {
        Ok(netifs) => {
            if let Some(netif) = netifs
                .values()
                .find(|net| !net.name.starts_with("lo") && !net.name.starts_with("docker"))
            {
                if let Some(addr) = netif.addrs.first() {
                    match addr.addr {
                        IpAddr::V4(v4) => format!("{v4}"),
                        _ => "Net: unknown".to_string(),
                    }
                } else {
                    "Net: no addr".to_string()
                }
            } else {
                "Net: no found".to_string()
            }
        }
        Err(x) => format!("Net: error: {x}"),
    }
}

pub fn format_uptime(up: Result<Duration>) -> String {
    match up {
        Ok(uptime) => format!("{}", crate::duration::FormattedDuration::new(uptime)),
        Err(x) => format!("Up: error: {x}"),
    }
}
