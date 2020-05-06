use enum_map::Enum;

use crate::telemetry::Telemetry;

#[derive(Copy, Clone, PartialEq, Enum)]
pub enum Align {
    Top,
    TopLeft,
    TopRight,
    Left,
    Center,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

pub trait Drawable<T: AsMut<[u8]>> {
    fn align(&self) -> Align;
    fn draw(&self, telemetry: &Telemetry, output: &mut [T]);
}
