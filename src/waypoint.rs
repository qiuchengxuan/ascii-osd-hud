use numtoa::NumToA;

use crate::drawable::{Align, Drawable, NumOfLine};
use crate::symbol::{to_number_with_dot, Symbol, SymbolIndex, SymbolTable};
use crate::telemetry::Telemetry;

pub struct Waypoint {
    zero_dot: SymbolIndex,
}

impl Waypoint {
    pub fn new(symbols: &SymbolTable) -> Self {
        Self {
            zero_dot: symbols[Symbol::ZeroWithTraillingDot],
        }
    }
}

impl<T: AsMut<[u8]>> Drawable<T> for Waypoint {
    fn align(&self) -> Align {
        Align::BottomRight
    }

    fn draw(&self, telemetry: &Telemetry, output: &mut [T]) -> NumOfLine {
        let last_index = output.len() - 1;
        let waypoint = &telemetry.waypoint;

        // TTG
        let buffer = output[last_index].as_mut();
        let buffer_len = buffer.len();
        let time_to_go = telemetry.time_to_go();
        let hours = (time_to_go / 3600) as u8;
        let minutes = (time_to_go / 60 % 60) as u8;
        let seconds = (time_to_go % 60) as u8;
        buffer[buffer_len - 8..].copy_from_slice(b"00:00:00");
        seconds.numtoa(10, buffer);
        minutes.numtoa(10, &mut buffer[..buffer_len - 3]);
        hours.numtoa(10, &mut buffer[..buffer_len - 6]);

        // distance
        let buffer = output[last_index - 1].as_mut();
        if waypoint.coordinate.rho < 100 {
            let rho = waypoint.coordinate.rho;
            rho.numtoa(10, &mut buffer[..buffer_len - 2]);
            buffer[buffer_len - 4] = to_number_with_dot(buffer[buffer_len - 4], self.zero_dot);
        } else {
            (waypoint.coordinate.rho / 10).numtoa(10, &mut buffer[..buffer_len - 2]);
        }
        buffer[buffer_len - 2..].copy_from_slice(&waypoint.unit[..]);

        // number and name
        let buffer = output[last_index - 2].as_mut();
        waypoint.number.numtoa(10, &mut buffer[..buffer_len - 5]);
        buffer[buffer_len - 5] = '/' as u8;
        buffer[buffer_len - 4..].copy_from_slice(&waypoint.name[..]);
        3
    }
}

#[cfg(test)]
mod test {
    use crate::drawable::Drawable;
    use crate::symbol::default_symbol_table;
    use crate::telemetry::Telemetry;
    use crate::test_utils::{to_utf8_string, ZeroSlice};

    use super::Waypoint;

    #[test]
    fn test_waypoint() {
        let mut buffer = [[0u8; 10]; 3];
        let waypoint = Waypoint::new(&default_symbol_table());
        let mut telemetry = Telemetry::default();
        waypoint.draw(&telemetry, &mut buffer);
        assert_eq!("    0/HOME      ₀0NM  00:00:00", to_utf8_string(&buffer));

        buffer.iter_mut().for_each(|b| b.zero());
        telemetry.waypoint.coordinate.rho = 600;
        waypoint.draw(&telemetry, &mut buffer);
        assert_eq!("    0/HOME      60NM  00:00:00", to_utf8_string(&buffer));

        buffer.iter_mut().for_each(|b| b.zero());
        telemetry.velocity_vector.rho = 60;
        waypoint.draw(&telemetry, &mut buffer);
        assert_eq!("    0/HOME      60NM  01:00:00", to_utf8_string(&buffer));

        buffer.iter_mut().for_each(|b| b.zero());
        telemetry.velocity_vector.rho = 61;
        waypoint.draw(&telemetry, &mut buffer);
        assert_eq!("    0/HOME      60NM  00:59:00", to_utf8_string(&buffer));

        buffer.iter_mut().for_each(|b| b.zero());
        telemetry.waypoint.coordinate.rho = 99;
        waypoint.draw(&telemetry, &mut buffer);
        assert_eq!("    0/HOME      ⒐9NM  00:09:44", to_utf8_string(&buffer));

        buffer.iter_mut().for_each(|b| b.zero());
        telemetry.waypoint.coordinate.rho = 98;
        waypoint.draw(&telemetry, &mut buffer);
        assert_eq!("    0/HOME      ⒐8NM  00:09:38", to_utf8_string(&buffer));
    }
}
