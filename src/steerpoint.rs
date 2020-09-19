use core::fmt::Write;

use heapless::consts::U8;
use heapless::String;

use crate::drawable::{Align, Drawable, NumOfLine};
use crate::symbol::{to_number_with_dot, Symbol, SymbolIndex, SymbolTable};
use crate::telemetry::Telemetry;

pub struct Steerpoint {
    zero_dot: SymbolIndex,
}

impl Steerpoint {
    pub fn new(symbols: &SymbolTable) -> Self {
        Self {
            zero_dot: symbols[Symbol::ZeroWithTraillingDot],
        }
    }
}

impl<T: AsMut<[u8]>> Drawable<T> for Steerpoint {
    fn align(&self) -> Align {
        Align::BottomRight
    }

    fn draw(&self, telemetry: &Telemetry, output: &mut [T]) -> NumOfLine {
        let last_index = output.len() - 1;
        let steerpoint = &telemetry.steerpoint;

        // TTG
        let buffer = output[last_index].as_mut();
        let buffer_len = buffer.len();
        let time_to_go = telemetry.time_to_go();
        let hours = (time_to_go / 3600) as u8;
        let minutes = (time_to_go / 60 % 60) as u8;
        let seconds = (time_to_go % 60) as u8;
        let mut string: String<U8> = String::new();
        write!(string, "{:02}:{:02}:{:02}", hours, minutes, seconds).ok();
        buffer[buffer_len - 8..].copy_from_slice(string.as_bytes());

        // distance
        let buffer = output[last_index - 1].as_mut();
        let rho = steerpoint.coordinate.rho;
        let mut string: String<U8> = String::new();
        if steerpoint.coordinate.rho < 100 {
            write!(string, "{}{}", rho, telemetry.unit.distance()).ok();
            let bytes = string.as_bytes();
            buffer[buffer_len - bytes.len()..].copy_from_slice(bytes);
            buffer[buffer_len - 4] = to_number_with_dot(buffer[buffer_len - 4], self.zero_dot);
        } else {
            write!(string, "{}{}", rho / 10, telemetry.unit.distance()).ok();
            let bytes = string.as_bytes();
            buffer[buffer_len - bytes.len()..].copy_from_slice(bytes);
        }

        // number and name
        let buffer = output[last_index - 2].as_mut();
        let mut string: String<U8> = String::new();
        write!(string, "{}/{:4}", steerpoint.number, steerpoint.name).ok();
        let bytes = string.as_bytes();
        buffer[buffer_len - bytes.len()..].copy_from_slice(bytes);
        3
    }
}

#[cfg(test)]
mod test {
    use crate::drawable::Drawable;
    use crate::symbol::default_symbol_table;
    use crate::telemetry::Telemetry;
    use crate::test_utils::{to_utf8_string, ZeroSlice};

    use super::Steerpoint;

    #[test]
    fn test_steerpoint() {
        let mut buffer = [[0u8; 10]; 3];
        let steerpoint = Steerpoint::new(&default_symbol_table());
        let mut telemetry = Telemetry::default();
        steerpoint.draw(&telemetry, &mut buffer);
        assert_eq!("    0/HOME      ₀0NM  00:00:00", to_utf8_string(&buffer));

        buffer.iter_mut().for_each(|b| b.zero());
        telemetry.steerpoint.coordinate.rho = 600;
        steerpoint.draw(&telemetry, &mut buffer);
        assert_eq!("    0/HOME      60NM  00:00:00", to_utf8_string(&buffer));

        buffer.iter_mut().for_each(|b| b.zero());
        telemetry.speed_vector.rho = 60;
        steerpoint.draw(&telemetry, &mut buffer);
        assert_eq!("    0/HOME      60NM  01:00:00", to_utf8_string(&buffer));

        buffer.iter_mut().for_each(|b| b.zero());
        telemetry.speed_vector.rho = 61;
        steerpoint.draw(&telemetry, &mut buffer);
        assert_eq!("    0/HOME      60NM  00:59:00", to_utf8_string(&buffer));

        buffer.iter_mut().for_each(|b| b.zero());
        telemetry.steerpoint.coordinate.rho = 99;
        steerpoint.draw(&telemetry, &mut buffer);
        assert_eq!("    0/HOME      ⒐9NM  00:09:44", to_utf8_string(&buffer));

        buffer.iter_mut().for_each(|b| b.zero());
        telemetry.steerpoint.coordinate.rho = 98;
        steerpoint.draw(&telemetry, &mut buffer);
        assert_eq!("    0/HOME      ⒐8NM  00:09:38", to_utf8_string(&buffer));
    }
}
