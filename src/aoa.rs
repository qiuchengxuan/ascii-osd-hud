use numtoa::NumToA;

use crate::drawable::{Align, Drawable};
use crate::symbol::{Symbol, SymbolIndex, SymbolTable};
use crate::telemetry::Telemetry;

pub struct AOA {
    alpha: SymbolIndex,
    zero_dot: SymbolIndex,
}

impl AOA {
    pub fn new(symbols: &SymbolTable) -> AOA {
        AOA {
            alpha: symbols[Symbol::Alpha],
            zero_dot: symbols[Symbol::ZeroWithTraillingDot],
        }
    }
}

impl<T: AsMut<[u8]>> Drawable<T> for AOA {
    fn align(&self) -> Align {
        Align::Left
    }

    fn draw(&self, telemetry: &Telemetry, output: &mut [T]) {
        let mut num_buffer: [u8; 5] = [0; 5];
        let num_str = telemetry.aoa.numtoa(10, &mut num_buffer);
        let buffer = output[0].as_mut();
        buffer[0] = self.alpha;
        buffer[2..2 + num_str.len()].copy_from_slice(num_str);
        if self.zero_dot > '0' as u8 {
            buffer[num_str.len()] += '0' as u8 - self.zero_dot;
        } else {
            buffer[num_str.len()] -= '0' as u8 - self.zero_dot;
        }
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
        let mut buffer = [[0u8; 4]];
        let aoa = AOA::new(&default_symbol_table());
        let mut telemetry = Telemetry::default();
        telemetry.aoa = 31;
        aoa.draw(&telemetry, &mut buffer);
        assert_eq!("⍺ ₃1", to_utf8_string(&buffer));
    }
}
