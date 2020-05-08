use crate::drawable::{Align, Drawable, NumOfLine};
use crate::telemetry::Telemetry;

pub struct FlightMode;

impl Default for FlightMode {
    fn default() -> Self {
        Self {}
    }
}

impl<T: AsMut<[u8]>> Drawable<T> for FlightMode {
    fn align(&self) -> Align {
        Align::Left
    }

    fn draw(&self, telemetry: &Telemetry, output: &mut [T]) -> NumOfLine {
        let buffer = output.last_mut().unwrap().as_mut();
        buffer[..4].copy_from_slice(&telemetry.flight_mode[..]);
        1
    }
}
