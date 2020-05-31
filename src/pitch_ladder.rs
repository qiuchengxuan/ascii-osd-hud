use crate::drawable::{Align, Drawable, NumOfLine};
use crate::symbol::{Symbol, SymbolIndex, SymbolTable};
use crate::telemetry::Telemetry;
use crate::AspectRatio;
#[allow(unused)] // false warning
use micromath::F32Ext;

pub struct Pitchladder {
    horizental_symbols: [SymbolIndex; 7],
    vertical_symbols: [SymbolIndex; 5],
    fov_height: isize,
}

const DEGREE_PER_RAD: f32 = 180.0 / core::f32::consts::PI;

type Point = (isize, isize);

impl Pitchladder {
    pub fn new(symbol_table: &SymbolTable, fov: u8, aspect_ratio: AspectRatio) -> Self {
        let mut ladder = Self {
            horizental_symbols: [0; 7],
            vertical_symbols: [0; 5],
            fov_height: aspect_ratio.diagonal_to_height(fov.into()) as isize,
        };
        let slice = &symbol_table.as_slice();
        let symbols = &slice[Symbol::LineTop as usize..Symbol::LineBottom as usize + 1];
        ladder.horizental_symbols.copy_from_slice(&symbols);
        let symbols = &slice[Symbol::LineLeft as usize..Symbol::LineRight as usize + 1];
        ladder.vertical_symbols.copy_from_slice(&symbols);
        ladder
    }

    fn draw_line<F: FnMut(isize, isize)>(&self, p0: Point, p1: Point, mut callback: F) {
        let (x0, y0) = p0;
        let (x1, y1) = p1;
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let (mut x, mut y) = (x0, y0);
        loop {
            callback(x, y);
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

        let mut roll = telemetry.attitude.roll;
        if roll >= 90 {
            roll = 90
        } else if roll <= -90 {
            roll = 89
        }
        let pitch = telemetry.attitude.pitch as isize;

        let k1000 = ((roll as f32 / DEGREE_PER_RAD).tan() * 1000.0) as isize; // y / x

        if -60 <= roll && roll <= 60 {
            let symbols = &self.horizental_symbols;
            let callback = |x, y| {
                let y_index = y / symbols.len() as isize;
                if 0 <= y_index && y_index < height && 0 <= x && x < width {
                    let symbol = symbols[y as usize % symbols.len()];
                    output[y_index as usize].as_mut()[x as usize] = symbol;
                }
            };
            let num_symbols = symbols.len() as isize;
            let y_offset = pitch * height * num_symbols / self.fov_height + num_symbols / 2;
            let y = ((width / 2 * height * num_symbols / width) * k1000 / 1000) as isize;
            let y0 = -y + (height / 2) * num_symbols + y_offset;
            let y1 = y + (height / 2) * num_symbols + y_offset;
            self.draw_line((0, y0), (width, y1), callback);
        } else {
            let symbols = &self.vertical_symbols;
            let callback = |y, x| {
                let x_index = x / symbols.len() as isize;
                if 0 <= x_index && x_index < width && 0 <= y && y < height {
                    let symbol = symbols[x as usize % symbols.len()];
                    output[y as usize].as_mut()[x_index as usize] = symbol;
                }
            };
            let num_symbols = symbols.len() as isize;
            let y_offset = pitch * height * num_symbols / self.fov_height;
            let x = ((height + y_offset) / 2 * width * num_symbols / height) * 1000 / k1000;
            let x0 = -x + (width / 2) * num_symbols + num_symbols / 2;
            let x1 = x + (width / 2) * num_symbols + num_symbols / 2;
            self.draw_line((0, x0), (height, x1), callback);
        }
        0
    }
}

#[cfg(test)]
mod test {
    use crate::drawable::Drawable;
    use crate::symbol::default_symbol_table;
    use crate::telemetry::Telemetry;
    use crate::test_utils::{fill_edge, to_utf8_string};
    use crate::AspectRatio;

    use super::Pitchladder;

    #[test]
    fn test_horizental() {
        let mut buffer = [[0u8; 32]; 9];
        let pitch_ladder = Pitchladder::new(&default_symbol_table(), 18, aspect_ratio!(16:9));
        let telemetry = Telemetry::default();
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
    }

    #[test]
    fn test_pitch() {
        let mut buffer = [[0u8; 32]; 9];
        let pitch_ladder = Pitchladder::new(&default_symbol_table(), 150, aspect_ratio!(16:9));
        let mut telemetry = Telemetry::default();
        telemetry.attitude.pitch = 7;
        pitch_ladder.draw(&telemetry, &mut buffer);
        fill_edge(&mut buffer);
        let expected = ".                              .\
                        .                              .\
                        .                              .\
                        .                              .\
                        .                              .\
                        ⎻⎻⎻⎻⎻⎻⎻⎻⎻⎻⎻⎻⎻⎻⎻⎻⎻⎻⎻⎻⎻⎻⎻⎻⎻⎻⎻⎻⎻⎻⎻⎻\
                        .                              .\
                        .                              .\
                        .                              .";
        assert_eq!(expected, to_utf8_string(&buffer));
    }

    #[test]
    fn test_shallow_roll_left() {
        let mut buffer = [[0u8; 32]; 9];
        let pitch_ladder = Pitchladder::new(&default_symbol_table(), 18, aspect_ratio!(16:9));
        let mut telemetry = Telemetry::default();
        telemetry.attitude.roll = -15;
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
    }

    #[test]
    fn test_shallow_roll_right() {
        let mut buffer = [[0u8; 32]; 9];
        let pitch_ladder = Pitchladder::new(&default_symbol_table(), 18, aspect_ratio!(16:9));
        let mut telemetry = Telemetry::default();
        telemetry.attitude.roll = 15;
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
    }

    #[test]
    fn test_roll_left() {
        let mut buffer = [[0u8; 32]; 9];
        let pitch_ladder = Pitchladder::new(&default_symbol_table(), 18, aspect_ratio!(16:9));
        let mut telemetry = Telemetry::default();
        telemetry.attitude.roll = -30;
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
    }

    #[test]
    fn test_roll_right() {
        let mut buffer = [[0u8; 32]; 9];
        let pitch_ladder = Pitchladder::new(&default_symbol_table(), 18, aspect_ratio!(16:9));
        let mut telemetry = Telemetry::default();
        telemetry.attitude.roll = 45;
        pitch_ladder.draw(&telemetry, &mut buffer);
        fill_edge(&mut buffer);
        let expected = "▔⎻⎼▁                           .\
                        .   ⎺─⎽▁                       .\
                        .      ▔⎻⎼▁                    .\
                        .          ⎺─⎽▁                .\
                        .             ▔⎻─⎽▁            .\
                        .                 ▔⎻⎼▁         .\
                        .                     ⎺─⎽▁     .\
                        .                        ▔⎻⎼▁  .\
                        .                            ⎺─⎽";
        assert_eq!(expected, to_utf8_string(&buffer));
    }

    #[test]
    fn test_deep_roll_left() {
        let mut buffer = [[0u8; 32]; 9];
        let pitch_ladder = Pitchladder::new(&default_symbol_table(), 18, aspect_ratio!(16:9));
        let mut telemetry = Telemetry::default();
        telemetry.attitude.roll = -80;
        pitch_ladder.draw(&telemetry, &mut buffer);
        fill_edge(&mut buffer);
        let expected = ".                 ⎪            .\
                        .                 ▏            .\
                        .                ⎪▏            .\
                        .                ▏             .\
                        .               ⎪              .\
                        .               ▏              .\
                        .              |               .\
                        .              ▏               .\
                        .             |                .";
        assert_eq!(expected, to_utf8_string(&buffer));
    }

    #[test]
    fn test_vertical() {
        let mut buffer = [[0u8; 32]; 9];
        let pitch_ladder = Pitchladder::new(&default_symbol_table(), 18, aspect_ratio!(16:9));
        let mut telemetry = Telemetry::default();
        telemetry.attitude.roll = 90;
        pitch_ladder.draw(&telemetry, &mut buffer);
        fill_edge(&mut buffer);
        let expected = ".               |              .\
                        .               |              .\
                        .               |              .\
                        .               |              .\
                        .               |              .\
                        .               |              .\
                        .               |              .\
                        .               |              .\
                        .               |              .";
        assert_eq!(expected, to_utf8_string(&buffer));
    }

    #[test]
    fn test_ranges() {
        let mut telemetry = Telemetry::default();
        let mut buffer = [[0u8; 32]; 9];
        let pitch_ladder = Pitchladder::new(&default_symbol_table(), 18, aspect_ratio!(16:9));
        for i in 0..180 {
            telemetry.attitude.roll = i as i8;
            pitch_ladder.draw(&telemetry, &mut buffer);
        }
    }
}
