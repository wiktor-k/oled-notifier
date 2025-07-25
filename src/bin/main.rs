use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics_core::draw_target::DrawTarget;
use linux_embedded_hal::I2cdev;
use ssd1306::mode::DisplayConfig;
use ssd1306::{I2CDisplayInterface, Ssd1306, rotation::DisplayRotation, size::DisplaySize128x64};
use systemstat::{Platform, System};

fn main() -> Result<(), Box<display_interface::DisplayError>> {
    let sys = System::new();

    let i2c = I2cdev::new("/dev/i2c-0").unwrap();

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate180)
        .into_buffered_graphics_mode();
    display.init()?;

    loop {
        display.clear(BinaryColor::Off)?;
        oled_notifier::draw_frame(&mut display, &sys)?;
        display.flush()?;
    }
}
