use numtoa::NumToA;

use crate::drawable::{Align, Drawable};
use crate::symbol::{Symbol, SymbolIndex, SymbolTable};
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

    fn draw(&self, telemetry: &Telemetry, output: &mut [T]) {
        let buffer = output[0].as_mut();
        telemetry.aoa.numtoa(10, &mut buffer[2..5]);
        buffer[0] = self.alpha;
        if '0' as u8 <= buffer[3] && buffer[3] <= '9' as u8 {
            if self.zero_dot > '0' as u8 {
                buffer[3] += self.zero_dot - '0' as u8;
            } else {
                buffer[3] -= '0' as u8 - self.zero_dot;
            }
        } else if buffer[3] == 0 {
            buffer[3] = self.zero_dot;
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
        let mut buffer = [[0u8; 6]];
        let aoa = AOA::new(&default_symbol_table());
        let mut telemetry = Telemetry::default();
        telemetry.aoa = 31;
        aoa.draw(&telemetry, &mut buffer);
        assert_eq!("⍺  ₃1 ", to_utf8_string(&buffer));

        buffer[0].iter_mut().for_each(|x| *x = 0);
        telemetry.aoa = 1;
        aoa.draw(&telemetry, &mut buffer);
        assert_eq!("⍺  ₀1 ", to_utf8_string(&buffer));
    }
}
