use core::fmt::Write;

use heapless::consts::U4;
use heapless::String;

use crate::drawable::{Align, Drawable, NumOfLine};
use crate::symbol::{to_number_with_dot, Symbol, SymbolIndex, SymbolTable};
use crate::telemetry::Telemetry;

pub struct AOA {
    alpha: SymbolIndex,
    zero_dot: SymbolIndex,
}

impl AOA {
    pub fn new(symbols: &SymbolTable) -> Self {
        Self {
            alpha: symbols[Symbol::Alpha],
            zero_dot: symbols[Symbol::ZeroWithTraillingDot],
        }
    }
}

impl<T: AsMut<[u8]>> Drawable<T> for AOA {
    fn align(&self) -> Align {
        Align::Left
    }

    fn draw(&self, telemetry: &Telemetry, output: &mut [T]) -> NumOfLine {
        let buffer = output[0].as_mut();
        buffer[0] = self.alpha;
        let mut string: String<U4> = String::new();
        write!(string, "{:4}", telemetry.aoa).ok();
        let bytes = string.as_bytes();
        buffer[1..5].copy_from_slice(bytes);
        if -10 < telemetry.aoa && telemetry.aoa < 0 {
            buffer[2] = b'-';
        }
        buffer[3] = to_number_with_dot(buffer[3], self.zero_dot);
        1
    }
}

#[cfg(test)]
mod test {
    use crate::drawable::Drawable;
    use crate::symbol::default_symbol_table;
    use crate::telemetry::Telemetry;
    use crate::test_utils::to_utf8_string;

    use super::AOA;

    #[test]
    fn test_aoa() {
        let mut buffer = [[0u8; 6]];
        let aoa = AOA::new(&default_symbol_table());
        let mut telemetry = Telemetry::default();
        telemetry.aoa = 31;
        aoa.draw(&telemetry, &mut buffer);
        assert_eq!("⍺  ⒊1 ", to_utf8_string(&buffer));

        buffer[0].iter_mut().for_each(|x| *x = 0);
        telemetry.aoa = -1;
        aoa.draw(&telemetry, &mut buffer);
        assert_eq!("⍺ -₀1 ", to_utf8_string(&buffer));
    }
}
