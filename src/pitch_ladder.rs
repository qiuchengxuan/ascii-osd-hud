#[allow(unused)] // false warning
use micromath::F32Ext;

use crate::drawable::{Align, Drawable, NumOfLine};
use crate::symbol::{Symbol, SymbolIndex, SymbolTable};
use crate::telemetry::Telemetry;
use crate::AspectRatio;

const SMALL_BLACK_SQUARE: usize = 7;
const VERTICAL_LINE: usize = 8;

pub struct Pitchladder {
    symbols: [SymbolIndex; 9],
    fov_height: isize,
}

type Point = (isize, isize);

impl Pitchladder {
    pub fn new(symbol_table: &SymbolTable, fov: u8, aspect_ratio: AspectRatio) -> Self {
        let mut symbols: [SymbolIndex; 9] = [0; 9];
        let slice = &symbol_table.as_slice();
        let horizen_symbols = &slice[Symbol::LineTop as usize..Symbol::VerticalLine as usize + 1];
        &symbols[..].copy_from_slice(&horizen_symbols);
        Self {
            symbols,
            fov_height: aspect_ratio.diagonal_to_height(fov.into()) as isize,
        }
    }

    fn draw_line<T: AsMut<[u8]>>(&self, p0: Point, p1: Point, symbols: &[u8], output: &mut [T]) {
        let (x0, y0) = p0;
        let (x1, y1) = p1;
        let height = output.len() as isize;
        let width = output[0].as_mut().len() as isize;
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let (mut x, mut y) = (x0, y0);
        loop {
            let y_index = y / symbols.len() as isize;
            if 0 <= y_index && y_index < height && 0 <= x && x < width {
                let symbol = symbols[y as usize % symbols.len()];
                output[y_index as usize].as_mut()[x as usize] = symbol;
            }
            if x == x1 && y == y1 {
                break;
            }
            let err2 = err * 2;
            if err2 >= dy {
                err += dy;
                x += sx;
            }
            if err2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }
}

impl<T: AsMut<[u8]>> Drawable<T> for Pitchladder {
    fn align(&self) -> Align {
        Align::Center
    }

    fn draw(&self, telemetry: &Telemetry, output: &mut [T]) -> NumOfLine {
        let height = output.len() as isize;
        let width = output[0].as_mut().len() as isize;

        let roll = telemetry.attitude.roll % 91;
        let k = (-roll as f32 / 57.3).tan(); // y / x
        let y_offset = telemetry.attitude.pitch as isize * height / self.fov_height;

        let symbols = if roll == 90 {
            &self.symbols[VERTICAL_LINE..VERTICAL_LINE + 1]
        } else if roll >= 60 || roll <= -60 {
            &self.symbols[SMALL_BLACK_SQUARE..SMALL_BLACK_SQUARE + 1]
        } else {
            &self.symbols[..SMALL_BLACK_SQUARE]
        };
        let num_symbols = symbols.len() as isize;

        let y = ((width / 2 * height * num_symbols / width) as f32 * k) as isize;
        let y0 = -y + (height / 2 + y_offset) * num_symbols + num_symbols / 2;
        let y1 = y + (height / 2 + y_offset) * num_symbols + num_symbols / 2;
        self.draw_line((0, y0), (width, y1), symbols, output);
        0
    }
}

#[cfg(test)]
mod test {
    use crate::drawable::Drawable;
    use crate::symbol::default_symbol_table;
    use crate::telemetry::Telemetry;
    use crate::test_utils::{fill_edge, to_utf8_string, ZeroSlice};
    use crate::AspectRatio;

    use super::Pitchladder;

    #[test]
    fn test_pitch_ladder() {
        let mut buffer = [[0u8; 32]; 9];
        let pitch_ladder = Pitchladder::new(&default_symbol_table(), 18, AspectRatio::Wide);
        let mut telemetry = Telemetry::default();
        pitch_ladder.draw(&telemetry, &mut buffer);
        fill_edge(&mut buffer);
        let expected = ".                              .\
                        .                              .\
                        .                              .\
                        .                              .\
                        ────────────────────────────────\
                        .                              .\
                        .                              .\
                        .                              .\
                        .                              .";
        assert_eq!(expected, to_utf8_string(&buffer));

        buffer.iter_mut().for_each(|b| b.zero());
        telemetry.attitude.roll = 15;
        pitch_ladder.draw(&telemetry, &mut buffer);
        fill_edge(&mut buffer);
        let expected = ".                              .\
                        .                              .\
                        .                              .\
                        .                      ▁▁⎽⎽⎼⎼──⎻\
                        .        ▁▁⎽⎽⎼⎼──⎻⎻⎺⎺▔▔        .\
                        ⎼──⎻⎻⎺⎺▔▔                      .\
                        .                              .\
                        .                              .\
                        .                              .";
        assert_eq!(expected, to_utf8_string(&buffer));

        buffer.iter_mut().for_each(|b| b.zero());
        telemetry.attitude.roll = -15;
        pitch_ladder.draw(&telemetry, &mut buffer);
        fill_edge(&mut buffer);
        let expected = ".                              .\
                        .                              .\
                        .                              .\
                        ⎻──⎼⎼⎽⎽▁▁                      .\
                        .        ▔▔⎺⎺⎻⎻──⎼⎼⎽⎽▁▁        .\
                        .                      ▔▔⎺⎺⎻⎻──⎼\
                        .                              .\
                        .                              .\
                        .                              .";
        assert_eq!(expected, to_utf8_string(&buffer));

        buffer.iter_mut().for_each(|b| b.zero());
        telemetry.attitude.roll = 30;
        pitch_ladder.draw(&telemetry, &mut buffer);
        fill_edge(&mut buffer);
        let expected = ".                              .\
                        .                              .\
                        .                         ▁⎽⎼─⎻⎺\
                        .                   ▁⎽⎼─⎺▔     .\
                        .            ▁⎽⎼─⎻⎺▔           .\
                        .      ▁⎼─⎻⎺▔                  .\
                        ▁⎽⎼─⎻⎺▔                        .\
                        .                              .\
                        .                              .";
        assert_eq!(expected, to_utf8_string(&buffer));

        buffer.iter_mut().for_each(|b| b.zero());
        telemetry.attitude.roll = -45;
        pitch_ladder.draw(&telemetry, &mut buffer);
        fill_edge(&mut buffer);
        let expected = "⎺─⎽▁                           .\
                        .  ▔⎻⎼▁                        .\
                        .      ⎺⎻⎼▁                    .\
                        .          ⎺─⎽▁                .\
                        .             ▔⎻─⎽▁            .\
                        .                 ▔⎻⎼▁         .\
                        .                     ⎺─⎼▁     .\
                        .                         ⎺─⎽▁ .\
                        .                            ▔⎻⎼";
        assert_eq!(expected, to_utf8_string(&buffer));

        buffer.iter_mut().for_each(|b| b.zero());
        telemetry.attitude.roll = 80;
        pitch_ladder.draw(&telemetry, &mut buffer);
        fill_edge(&mut buffer);
        let expected = ".                  ▪           .\
                        .                 ▪            .\
                        .                ▪             .\
                        .                ▪             .\
                        .               ▪              .\
                        .              ▪               .\
                        .              ▪               .\
                        .             ▪                .\
                        .            ▪                 .";
        assert_eq!(expected, to_utf8_string(&buffer));

        buffer.iter_mut().for_each(|b| b.zero());
        telemetry.attitude.roll = 90;
        pitch_ladder.draw(&telemetry, &mut buffer);
        fill_edge(&mut buffer);
        let expected = ".               │              .\
                        .               │              .\
                        .               │              .\
                        .               │              .\
                        .               │              .\
                        .               │              .\
                        .               │              .\
                        .               │              .\
                        .               │              .";
        assert_eq!(expected, to_utf8_string(&buffer));
    }

    #[test]
    fn test_ranges() {
        let mut telemetry = Telemetry::default();
        let mut buffer = [[0u8; 32]; 9];
        let pitch_ladder = Pitchladder::new(&default_symbol_table(), 18, AspectRatio::Wide);
        for i in 0..180 {
            telemetry.attitude.roll = i as i8;
            pitch_ladder.draw(&telemetry, &mut buffer);
        }
    }
}
