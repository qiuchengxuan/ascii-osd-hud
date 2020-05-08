use crate::drawable::{Align, Drawable, NumOfLine};
use crate::symbol::{Symbol, SymbolTable};
use crate::telemetry::Telemetry;

pub struct Pitchladder<'a> {
    symbols: &'a SymbolTable,
    fov: u8,
    aspect_ratio: (u8, u8),
}

impl<'a> Pitchladder<'a> {
    pub fn new(symbols: &'a SymbolTable, fov: u8, aspect_ratio: (u8, u8)) -> Self {
        Self {
            symbols,
            fov,
            aspect_ratio,
        }
    }
}
impl<'a, T: AsMut<[u8]>> Drawable<T> for Pitchladder<'a> {
    fn align(&self) -> Align {
        Align::Center
    }

    fn draw(&self, telemetry: &Telemetry, output: &mut [T]) -> NumOfLine {
        0
    }
}
