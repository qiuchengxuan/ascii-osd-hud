use crate::telemetry::Telemetry;

pub enum Layer {
    Top,
    Bottom,
}

#[derive(Copy, Clone, PartialEq)]
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

pub trait Drawable {
    fn layer(&self) -> Layer;
    fn align(&self) -> Align;
    fn draw<T: AsMut<[u8]>>(&self, telemetry: &Telemetry, output: &mut [T]);
}
