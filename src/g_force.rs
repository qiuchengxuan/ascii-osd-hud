use numtoa::NumToA;

use crate::drawable::{Align, Drawable};
use crate::symbol::{Symbol, SymbolIndex, SymbolTable};
use crate::telemetry::Telemetry;

pub struct GForce {
    zero_dot: SymbolIndex,
}

impl GForce {
    pub fn new(symbols: &SymbolTable) -> Self {
        Self {
            zero_dot: symbols[Symbol::ZeroWithTraillingDot],
        }
    }
}

impl<T: AsMut<[u8]>> Drawable<T> for GForce {
    fn align(&self) -> Align {
        Align::Left
    }

    fn draw(&self, telemetry: &Telemetry, output: &mut [T]) {
        let buffer = output[0].as_mut();
        telemetry.g_force.numtoa(10, &mut buffer[2..5]);
        buffer[0] = 'g' as u8;
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

    use super::GForce;

    #[test]
    fn test_g_force() {
        let mut buffer = [[0u8; 6]];
        let g_force = GForce::new(&default_symbol_table());
        let mut telemetry = Telemetry::default();
        telemetry.g_force = 11;
        g_force.draw(&telemetry, &mut buffer);
        assert_eq!("g  ₁1 ", to_utf8_string(&buffer));

        buffer[0].iter_mut().for_each(|x| *x = 0);
        telemetry.g_force = 9;
        g_force.draw(&telemetry, &mut buffer);
        assert_eq!("g  ₀9 ", to_utf8_string(&buffer));
    }
}
