use numtoa::NumToA;

use crate::drawable::{Align, Drawable, NumOfLine};
use crate::symbol::{to_number_with_dot, Symbol, SymbolIndex, SymbolTable};
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

    fn draw(&self, telemetry: &Telemetry, output: &mut [T]) -> NumOfLine {
        let buffer = output[0].as_mut();
        telemetry.g_force.numtoa(10, &mut buffer[2..5]);
        buffer[0] = 'g' as u8;
        buffer[1..3].iter_mut().for_each(|b| *b = ' ' as u8);
        buffer[3] = to_number_with_dot(buffer[3], self.zero_dot);
        1
    }
}

#[cfg(test)]
mod test {
    use crate::drawable::Drawable;
    use crate::symbol::default_symbol_table;
    use crate::telemetry::Telemetry;
    use crate::test_utils::{to_utf8_string, ZeroSlice};

    use super::GForce;

    #[test]
    fn test_g_force() {
        let mut buffer = [[0u8; 6]];
        let g_force = GForce::new(&default_symbol_table());
        let mut telemetry = Telemetry::default();
        telemetry.g_force = 11;
        g_force.draw(&telemetry, &mut buffer);
        assert_eq!("g  ⒈1 ", to_utf8_string(&buffer));

        buffer[0].zero();
        telemetry.g_force = 9;
        g_force.draw(&telemetry, &mut buffer);
        assert_eq!("g  ₀9 ", to_utf8_string(&buffer));
    }
}
