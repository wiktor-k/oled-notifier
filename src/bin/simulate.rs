#![cfg(feature = "simulator")]

use embedded_graphics_core::{pixelcolor::BinaryColor, prelude::*};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};
use systemstat::{DelayedMeasurement, Platform, PlatformCpuLoad};
use unimpl::unimpl;

struct FakeSystem;

impl Platform for FakeSystem {
    fn new() -> Self {
        Self
    }

    fn cpu_load(
        &self,
    ) -> std::io::Result<systemstat::DelayedMeasurement<Vec<systemstat::CPULoad>>> {
        Ok(DelayedMeasurement::new(Box::new(|| {
            Ok(vec![
                systemstat::CPULoad {
                    user: 0.004972356,
                    nice: Default::default(),
                    system: Default::default(),
                    interrupt: Default::default(),
                    idle: Default::default(),
                    platform: PlatformCpuLoad {
                        iowait: Default::default(),
                    },
                },
                systemstat::CPULoad {
                    user: 0.0,
                    nice: Default::default(),
                    system: Default::default(),
                    interrupt: Default::default(),
                    idle: Default::default(),
                    platform: PlatformCpuLoad {
                        iowait: Default::default(),
                    },
                },
                systemstat::CPULoad {
                    user: 0.03267,
                    nice: Default::default(),
                    system: Default::default(),
                    interrupt: Default::default(),
                    idle: Default::default(),
                    platform: PlatformCpuLoad {
                        iowait: Default::default(),
                    },
                },
            ])
        })))
    }

    #[unimpl]
    fn load_average(&self) -> std::io::Result<systemstat::LoadAverage>;

    fn memory(&self) -> std::io::Result<systemstat::Memory> {
        Ok(systemstat::Memory {
            total: systemstat::data::ByteSize::mb(123),
            free: systemstat::data::ByteSize::mb(12),
            platform_memory: systemstat::data::PlatformMemory {
                meminfo: std::collections::BTreeMap::default(),
            },
        })
    }
    #[unimpl]
    fn battery_life(&self) -> std::io::Result<systemstat::BatteryLife>;

    #[unimpl]
    fn on_ac_power(&self) -> std::io::Result<bool>;

    #[unimpl]
    fn mounts(&self) -> std::io::Result<Vec<systemstat::Filesystem>>;

    #[unimpl]
    fn block_device_statistics(
        &self,
    ) -> std::io::Result<systemstat::BTreeMap<String, systemstat::BlockDeviceStats>>;

    fn networks(&self) -> std::io::Result<systemstat::BTreeMap<String, systemstat::Network>> {
        let mut map = systemstat::BTreeMap::new();
        map.insert(
            "docker0".into(),
            systemstat::Network {
                name: "docker0".into(),
                addrs: vec![systemstat::data::NetworkAddrs {
                    addr: systemstat::data::IpAddr::V4("1.0.0.1".parse().unwrap()),
                    netmask: systemstat::data::IpAddr::V4("0.0.0.0".parse().unwrap()),
                }],
            },
        );
        map.insert(
            "lo".into(),
            systemstat::Network {
                name: "lo".into(),
                addrs: vec![systemstat::data::NetworkAddrs {
                    addr: systemstat::data::IpAddr::V4("2.0.0.1".parse().unwrap()),
                    netmask: systemstat::data::IpAddr::V4("0.0.0.0".parse().unwrap()),
                }],
            },
        );
        map.insert(
            "eth0".into(),
            systemstat::Network {
                name: "eth0".into(),
                addrs: vec![systemstat::data::NetworkAddrs {
                    addr: systemstat::data::IpAddr::V4("127.0.0.1".parse().unwrap()),
                    netmask: systemstat::data::IpAddr::V4("0.0.0.0".parse().unwrap()),
                }],
            },
        );
        Ok(map)
    }

    fn uptime(&self) -> std::io::Result<std::time::Duration> {
        Ok(std::time::Duration::from_secs(9987))
    }

    #[unimpl]
    fn network_stats(&self, _interface: &str) -> std::io::Result<systemstat::NetworkStats>;

    #[unimpl]
    fn cpu_temp(&self) -> std::io::Result<f32>;

    #[unimpl]
    fn socket_stats(&self) -> std::io::Result<systemstat::SocketStats>;

    #[unimpl]
    fn swap(&self) -> std::io::Result<systemstat::Swap>;
}

fn main() -> Result<(), Box<core::convert::Infallible>> {
    let sys = FakeSystem::new();

    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(128, 64));

    oled_notifier::draw_frame(&mut display, &sys)?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::Default)
        .build();
    Window::new("Simulator", &output_settings).show_static(&display);

    Ok(())
}
