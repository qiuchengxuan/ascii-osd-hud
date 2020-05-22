use core::cell::Cell;

use crate::drawable::{Align, Drawable, NumOfLine};
use crate::symbol::{Symbol, SymbolIndex, SymbolTable};
use crate::telemetry::Telemetry;
use crate::AspectRatio;

pub struct WaypointVector {
    vector: SymbolIndex,
    fov_width: u8,
    fov_height: u8,
    counter: Cell<u8>,
}

impl WaypointVector {
    pub fn new(symbols: &SymbolTable, fov: u8, aspect_ratio: AspectRatio) -> Self {
        Self {
            vector: symbols[Symbol::Square],
            fov_width: aspect_ratio.diagonal_to_width(fov.into()) as u8,
            fov_height: aspect_ratio.diagonal_to_height(fov.into()) as u8,
            counter: Cell::new(0),
        }
    }
}

impl<T: AsMut<[u8]>> Drawable<T> for WaypointVector {
    fn align(&self) -> Align {
        Align::Center
    }

    fn draw(&self, telemetry: &Telemetry, output: &mut [T]) -> NumOfLine {
        let waypoint = &telemetry.waypoint.coordinate;
        let phi = -waypoint.phi as isize;
        let height = output.len() as isize;
        let mut y = phi * height / self.fov_height as isize + height / 2;
        if y < 0 {
            y = 0;
        } else if y >= height {
            y = height - 1;
        }
        let buffer = output[y as usize].as_mut();
        let width = buffer.len() as isize;

        let mut theta = waypoint.theta as isize;
        if theta > 180 {
            theta = theta - 360;
        }
        let mut x = theta * width / self.fov_width as isize + width / 2;
        if x < 0 {
            x = 0;
        } else if x >= width {
            x = width - 1;
        }
        let byte = buffer[x as usize];
        if byte == 0 || byte == ' ' as u8 || self.counter.get() % 2 == 1 {
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

    use super::WaypointVector;

    #[test]
    fn test_waypoint_vector() {
        let mut buffer = [[0u8; 32]; 9];
        let waypoint_vector = WaypointVector::new(&default_symbol_table(), 18, AspectRatio::Wide);
        let mut telemetry = Telemetry::default();
        telemetry.waypoint.coordinate.theta = 1;
        telemetry.waypoint.coordinate.phi = -1;
        waypoint_vector.draw(&telemetry, &mut buffer);
        fill_edge(&mut buffer);
        let expected = ".                              .\
                        .                              .\
                        .                              .\
                        .                              .\
                        .                              .\
                        .                 ☐            .\
                        .                              .\
                        .                              .\
                        .                              .";
        assert_eq!(expected, to_utf8_string(&buffer));

        buffer.iter_mut().for_each(|b| b.zero());
        telemetry.waypoint.coordinate.phi = 1;
        waypoint_vector.draw(&telemetry, &mut buffer);
        fill_edge(&mut buffer);
        let expected = ".                              .\
                        .                              .\
                        .                              .\
                        .                 ☐            .\
                        .                              .\
                        .                              .\
                        .                              .\
                        .                              .\
                        .                              .";
        assert_eq!(expected, to_utf8_string(&buffer));

        buffer.iter_mut().for_each(|b| b.zero());
        telemetry.waypoint.coordinate.theta = 45;
        telemetry.waypoint.coordinate.phi = -45;
        waypoint_vector.draw(&telemetry, &mut buffer);
        fill_edge(&mut buffer);
        let expected = ".                              .\
                        .                              .\
                        .                              .\
                        .                              .\
                        .                              .\
                        .                              .\
                        .                              .\
                        .                              .\
                        .                              ☐";
        assert_eq!(expected, to_utf8_string(&buffer));
    }
}
