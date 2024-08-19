use embassy_futures::select::select;
use embassy_stm32::exti::ExtiInput;
use embassy_time::Instant;

pub enum RotaryDirection {
  Clockwise,
  CounterClockwise,
}

pub struct RotaryKnob<'a> {
  key_a: ExtiInput<'a>,
  key_b: ExtiInput<'a>,
  last_state: (bool, bool),
}

impl<'a> RotaryKnob<'a> {
  pub fn new(key_a: ExtiInput<'a>, key_b: ExtiInput<'a>) -> Self {
    Self {
      key_a,
      key_b,
      last_state: (false, false),
    }
  }

  // TODO: This results in random directions occasionally... clean the signal and/or refactor to wait for high/low
  pub async fn wait_for_change(&mut self) -> Option<RotaryDirection> {
    select(self.key_a.wait_for_any_edge(), self.key_b.wait_for_any_edge()).await;
    let new_state = (self.key_a.is_high(), self.key_b.is_high());

    let direction = match (new_state, self.last_state) {
      ((true, true), (false, true)) | ((false, false), (true, false)) => Some(RotaryDirection::Clockwise),
      ((true, true), (true, false)) | ((false, false), (false, true)) => Some(RotaryDirection::CounterClockwise),
      _ => None,
    };

    self.last_state = new_state;
    direction
  }
}

pub enum ButtonEvent {
  Press,
  Hold,
}

pub struct PushButton<'a> {
  input: ExtiInput<'a>,
}

impl<'a> PushButton<'a> {
  pub fn new(input: ExtiInput<'a>) -> Self {
    Self { input }
  }

  pub async fn wait_for_event(&mut self) -> Option<ButtonEvent> {
    self.input.wait_for_falling_edge().await;
    let start = Instant::now();
    self.input.wait_for_rising_edge().await;
    let diff = Instant::now().checked_duration_since(start);
    match diff?.as_millis() {
      50..500 => Some(ButtonEvent::Press),
      _ => Some(ButtonEvent::Hold),
    }
  }
}
