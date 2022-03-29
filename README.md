# HC4 OLED Notifier

This application provides simple statistics for [ODROID-HC4 with OLED] device.

[ODROID-HC4 with OLED]: https://www.hardkernel.com/shop/odroid-hc4-oled/

![Real-life render](docs/real.jpg)

Currently there are no options and the device used (`/dev/i2c-0`) and
rotation (180 degrees) is hardcoded to match HC4 OLED's display. The
display is refreshed every 2 seconds.

## Building

```sh
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-Clink-self-contained=yes -Clinker=rust-lld"
export AR_aarch64_unknown_linux_musl=llvm-ar
export CC_aarch64_unknown_linux_musl=clang

cargo build --bin main --release --target aarch64-unknown-linux-musl
```

## Running

If there are no `/dev/i2c-0` device use the kernel module: `modprobe
i2c-dev`.

## End-to-end tests

The end-to-end tests render a frame using fake system statistics and
check if the image is consistent with the known-good output:

![Simulated output](docs/screenshot.png)

To regenerate end-to-end screenshots run the following command:

```sh
EG_SIMULATOR_DUMP=docs/screenshot.png cargo run --features simulator --bin simulate
```

## See also

  - [sys-oled-hc4](https://github.com/rpardini/sys-oled-hc4) for more
    features (Python).
