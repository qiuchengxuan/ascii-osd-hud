use numtoa::NumToA;

use crate::drawable::{Align, Drawable, NumOfLine};
use crate::symbol::{Symbol, SymbolIndex, SymbolTable};
use crate::telemetry::Telemetry;

pub struct Battery {
    battery: SymbolIndex,
}

impl Battery {
    pub fn new(symbols: &SymbolTable) -> Self {
        Self {
            battery: symbols[Symbol::Battery],
        }
    }
}

impl<T: AsMut<[u8]>> Drawable<T> for Battery {
    fn align(&self) -> Align {
        Align::TopRight
    }

    fn draw(&self, telemetry: &Telemetry, output: &mut [T]) -> NumOfLine {
        let mut num_buffer: [u8; 5] = [0; 5];
        let number = telemetry.battery.numtoa(10, &mut num_buffer);
        let buffer = output[0].as_mut();
        let buffer_len = buffer.len();
        buffer[buffer_len - number.len() - 1] = self.battery;
        buffer[buffer_len - number.len()..].copy_from_slice(number);
        1
    }
}

#[cfg(test)]
mod test {
    use crate::drawable::Drawable;
    use crate::symbol::default_symbol_table;
    use crate::telemetry::Telemetry;
    use crate::test_utils::to_utf8_string;

    use super::Battery;

    #[test]
    fn test_altitude() {
        let mut buffer = [[0u8; 4]];
        let battery = Battery::new(&default_symbol_table());
        let mut telemetry = Telemetry::default();
        telemetry.battery = 100;
        battery.draw(&telemetry, &mut buffer);
        assert_eq!("β100", to_utf8_string(&buffer));
    }
}
