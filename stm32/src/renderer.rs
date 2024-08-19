use embedded_graphics::{
  draw_target::DrawTarget,
  mono_font::{ascii::FONT_8X13_BOLD, MonoTextStyle},
  pixelcolor::Rgb565,
  prelude::*,
  primitives::{PrimitiveStyle, Rectangle},
  text::{Alignment, Text},
};

fn value_to_text(value: u16) -> &'static str {
  match value {
    2 => "2",
    4 => "4",
    8 => "8",
    16 => "16",
    32 => "32",
    64 => "64",
    128 => "128",
    256 => "256",
    512 => "512",
    1024 => "1024",
    2048 => "2048",
    _ => "",
  }
}

pub fn render_general_items<D: DrawTarget<Color = Rgb565>>(display: &mut D) -> Result<(), D::Error> {
  Rectangle::new(Point::new(38, 38), Size::new(164, 164))
    .into_styled(PrimitiveStyle::with_fill(Rgb565::CSS_SADDLE_BROWN))
    .draw(display)?;
  Ok(())
}

pub fn render_grid<D: DrawTarget<Color = Rgb565>>(display: &mut D, data: [[u16; 4]; 4]) -> Result<(), D::Error> {
  let cell_style = PrimitiveStyle::with_fill(Rgb565::CSS_WHEAT);
  let text_style = MonoTextStyle::new(&FONT_8X13_BOLD, Rgb565::CSS_BLACK);
  for x in 0..4u8 {
    for y in 0..4u8 {
      let cell_position = Point::new(40 * (x as i32) + 1 + 40, 40 * (y as i32) + 1 + 40);
      Rectangle::new(cell_position, Size::new(38, 38))
        .into_styled(cell_style)
        .draw(display)?;

      let cell_value = data[y as usize][x as usize];
      if cell_value != 0 {
        Text::with_alignment(
          value_to_text(cell_value),
          Point::new(cell_position.x + 20, cell_position.y + 24),
          text_style,
          Alignment::Center,
        )
        .draw(display)?;
      }
    }
  }

  Ok(())
}
