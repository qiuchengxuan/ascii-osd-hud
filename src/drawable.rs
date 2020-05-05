use crate::data_source::Data;

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
    fn draw<T: AsMut<[u8]>>(&self, data: &Data, output: &mut [T]);
}
