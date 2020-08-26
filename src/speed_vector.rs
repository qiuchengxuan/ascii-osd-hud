use core::cell::Cell;

use crate::drawable::{Align, Drawable, NumOfLine};
use crate::symbol::{Symbol, SymbolIndex, SymbolTable};
use crate::telemetry::Telemetry;
use crate::AspectRatio;

pub struct SpeedVector {
    vector: SymbolIndex,
    fov_width: u8,
    fov_height: u8,
    counter: Cell<u8>,
}

impl SpeedVector {
    pub fn new(symbols: &SymbolTable, fov: u8, aspect_ratio: AspectRatio) -> Self {
        Self {
            vector: symbols[Symbol::VeclocityVector],
            fov_width: aspect_ratio.diagonal_to_width(fov.into()) as u8,
            fov_height: aspect_ratio.diagonal_to_height(fov.into()) as u8,
            counter: Cell::new(0),
        }
    }
}

fn with_ratio(speed: isize, degree: isize) -> isize {
    if speed >= 5 {
        return degree as isize;
    }
    degree * speed * speed / 5 / 5
}

impl<T: AsMut<[u8]>> Drawable<T> for SpeedVector {
    fn align(&self) -> Align {
        Align::Center
    }

    fn draw(&self, telemetry: &Telemetry, output: &mut [T]) -> NumOfLine {
        let speed = telemetry.speed() as isize;
        let height = output.len() as isize;
        let y_degree = -with_ratio(speed, telemetry.speed_vector.phi as isize);
        let mut y = y_degree * height / self.fov_height as isize + height / 2;
        if y < 0 {
            y = 0;
        } else if y >= height {
            y = height - 1;
        }
        let buffer = output[y as usize].as_mut();
        let width = buffer.len() as isize;

        let azimuth = telemetry.speed_vector.theta as isize;
        let x_degree = with_ratio(speed, azimuth);
        let mut x = x_degree * width / self.fov_width as isize + width / 2;
        if x < 0 {
            x = 0;
        } else if x >= width {
            x = width - 1;
        }
        let byte = buffer[x as usize];
        if byte == 0 || byte == b' ' || self.counter.get() % 2 == 0 {
            buffer[x as usize] = self.vector;
        }
        self.counter.set(self.counter.get() + 1);
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

    use super::SpeedVector;

    #[test]
    fn test_speed_vector() {
        let mut buffer = [[0u8; 32]; 9];
        let speed_vector = SpeedVector::new(&default_symbol_table(), 18, aspect_ratio!(16:9));
        let mut telemetry = Telemetry::default();
        telemetry.speed_vector.theta = 1;
        telemetry.speed_vector.phi = -1;
        speed_vector.draw(&telemetry, &mut buffer);
        fill_edge(&mut buffer);
        let expected = ".                              .\
                        .                              .\
                        .                              .\
                        .                              .\
                        .               ⏂              .\
                        .                              .\
                        .                              .\
                        .                              .\
                        .                              .";
        assert_eq!(expected, to_utf8_string(&buffer));

        buffer.iter_mut().for_each(|b| b.zero());
        telemetry.speed_vector.rho = 5;
        speed_vector.draw(&telemetry, &mut buffer);
        fill_edge(&mut buffer);
        let expected = ".                              .\
                        .                              .\
                        .                              .\
                        .                              .\
                        .                              .\
                        .                 ⏂            .\
                        .                              .\
                        .                              .\
                        .                              .";
        assert_eq!(expected, to_utf8_string(&buffer));

        buffer.iter_mut().for_each(|b| b.zero());
        telemetry.speed_vector.theta = 45;
        telemetry.speed_vector.phi = -45;
        speed_vector.draw(&telemetry, &mut buffer);
        fill_edge(&mut buffer);
        let expected = ".                              .\
                        .                              .\
                        .                              .\
                        .                              .\
                        .                              .\
                        .                              .\
                        .                              .\
                        .                              .\
                        .                              ⏂";
        assert_eq!(expected, to_utf8_string(&buffer));
    }
}
