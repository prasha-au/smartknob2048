#![no_std]
#![no_main]

use defmt::*;
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_futures::select::{select, Either};
use embassy_stm32::{exti::ExtiInput, gpio, spi::Spi};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use embassy_time::Delay;

use display_interface_spi::SPIInterface;
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::Rgb565, prelude::WebColors};
use gc9a01a::GC9A01A;

use embedded_hal::PwmPin as IPwmPin;

mod hid;
mod renderer;

static ROTARY_SIGNAL: Signal<CriticalSectionRawMutex, hid::RotaryDirection> = Signal::new();
static BUTTON_SIGNAL: Signal<CriticalSectionRawMutex, hid::ButtonEvent> = Signal::new();

struct FakePwm {}
impl IPwmPin for FakePwm {
  type Duty = u32;
  fn disable(&mut self) {}
  fn enable(&mut self) {}
  fn get_duty(&self) -> u32 {
    0
  }
  fn get_max_duty(&self) -> u32 {
    0
  }
  fn set_duty(&mut self, _duty: u32) {}
}

#[embassy_executor::task]
async fn handle_rotary_knob(mut knob: hid::RotaryKnob<'static>) {
  loop {
    let change_option = knob.wait_for_change().await;
    if let Some(change) = change_option {
      ROTARY_SIGNAL.signal(change);
    }
  }
}

#[embassy_executor::task]
async fn handle_button_press(mut button: hid::PushButton<'static>) {
  loop {
    let event_option = button.wait_for_event().await;
    if let Some(event) = event_option {
      BUTTON_SIGNAL.signal(event);
    }
  }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
  let p = embassy_stm32::init(Default::default());
  info!("Hello World!");

  let rotary_knob = hid::RotaryKnob::new(
    ExtiInput::new(p.PA12, p.EXTI12, gpio::Pull::Up),
    ExtiInput::new(p.PA11, p.EXTI11, gpio::Pull::Up),
  );
  unwrap!(spawner.spawn(handle_rotary_knob(rotary_knob)));
  // loop {
  //   match ROTARY_SIGNAL.wait().await {
  //     hid::RotaryDirection::Clockwise => info!("Clockwise"),
  //     hid::RotaryDirection::CounterClockwise => info!("Counter Clockwise"),
  //   }
  // }

  let button = hid::PushButton::new(ExtiInput::new(p.PA15, p.EXTI15, gpio::Pull::Up));
  unwrap!(spawner.spawn(handle_button_press(button)));
  // loop {
  //   match BUTTON_SIGNAL.wait().await {
  //     hid::ButtonEvent::Press => info!("Press"),
  //     hid::ButtonEvent::Hold => info!("Hold"),
  //   }
  // }

  let spi_bus = Spi::new_blocking_txonly(p.SPI2, p.PB10, p.PB11, Default::default());

  let cs_pin = gpio::Output::new(p.PB12, gpio::Level::Low, gpio::Speed::Low);
  let dc_pin = gpio::Output::new(p.PB13, gpio::Level::Low, gpio::Speed::Low);
  let spi_interface = SPIInterface::new(spi_bus, dc_pin, cs_pin);

  let reset_pin = gpio::Output::new(p.PA8, gpio::Level::Low, gpio::Speed::VeryHigh);
  let mut display = GC9A01A::new(spi_interface, reset_pin, FakePwm {});

  let _bl_pin = gpio::Output::new(p.PC6, gpio::Level::Low, gpio::Speed::Low);

  display.reset(&mut Delay).unwrap();
  display.initialize(&mut Delay).unwrap();
  display.clear(Rgb565::CSS_BLACK).unwrap();

  renderer::render_general_items(&mut display).unwrap();

  let mut game = app::Game::create();
  game.add_tile();
  game.add_tile();

  info!("Ready to go");
  loop {
    renderer::render_grid(&mut display, game.board).unwrap();

    let result = select(ROTARY_SIGNAL.wait(), BUTTON_SIGNAL.wait()).await;

    match result {
      Either::First(direction) => {
        info!("Rotary");
        match direction {
          hid::RotaryDirection::Clockwise => game.move_right(),
          hid::RotaryDirection::CounterClockwise => game.move_left(),
        }

        match game.check_win_loss() {
          Some(true) => {
            info!("You won!");
            break;
          }
          Some(false) => {
            info!("You lost!");
            break;
          }
          None => {}
        }

        game.add_tile();
      }
      Either::Second(_) => {
        info!("Button pressed. Rotating");
        game.rotate();
      }
    }
  }
}
