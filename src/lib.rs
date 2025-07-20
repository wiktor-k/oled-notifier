#![doc = include_str!("../README.md")]

mod duration;
pub mod stats;

use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::mono_font::ascii::*;
use embedded_graphics::text::{Baseline, Text};
use embedded_graphics_core::{draw_target::DrawTarget, pixelcolor::BinaryColor, prelude::*};

use crate::stats::*;

use systemstat::Platform;

pub struct Error;

pub fn draw_frame<D, P>(display: &mut D, sys: &P) -> Result<(), D::Error>
where
    P: Platform,
    D: DrawTarget<Color = BinaryColor>,
{
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_8X13)
        .text_color(BinaryColor::On)
        .build();

    let cpu_load = format_cpu_load(sys.cpu_load());

    Text::with_baseline(&cpu_load, Point::zero(), text_style, Baseline::Top).draw(display)?;

    let mem = format_memory(sys.memory());

    Text::with_baseline(&mem, Point::new(0, 16), text_style, Baseline::Top).draw(display)?;

    let net = format_networks(sys.networks());

    Text::with_baseline(&net, Point::new(0, 32), text_style, Baseline::Top).draw(display)?;

    let up = format_uptime(sys.uptime());

    Text::with_baseline(&up, Point::new(0, 48), text_style, Baseline::Top).draw(display)?;

    Ok(())
}
