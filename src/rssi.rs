use numtoa::NumToA;

use crate::drawable::{Align, Drawable, NumOfLine};
use crate::symbol::{Symbol, SymbolIndex, SymbolTable};
use crate::telemetry::Telemetry;

pub struct RSSI {
    antenna: SymbolIndex,
}

impl RSSI {
    pub fn new(symbols: &SymbolTable) -> Self {
        Self {
            antenna: symbols[Symbol::Antenna],
        }
    }
}

impl<T: AsMut<[u8]>> Drawable<T> for RSSI {
    fn align(&self) -> Align {
        Align::TopLeft
    }

    fn draw(&self, telemetry: &Telemetry, output: &mut [T]) -> NumOfLine {
        let mut num_buffer: [u8; 5] = [0; 5];
        let num_str = telemetry.rssi.numtoa(10, &mut num_buffer);
        let buffer = output[0].as_mut();
        buffer[0] = self.antenna;
        buffer[1..1 + num_str.len()].copy_from_slice(num_str);
        1
    }
}

#[cfg(test)]
mod test {
    use crate::drawable::Drawable;
    use crate::symbol::default_symbol_table;
    use crate::telemetry::Telemetry;
    use crate::test_utils::to_utf8_string;

    use super::RSSI;

    #[test]
    fn test_rssi() {
        let mut buffer = [[0u8; 4]];
        let rssi = RSSI::new(&default_symbol_table());
        let mut telemetry = Telemetry::default();
        telemetry.rssi = 100;
        rssi.draw(&telemetry, &mut buffer);
        assert_eq!("⏉100", to_utf8_string(&buffer));
    }
}
