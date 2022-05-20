use core::fmt::Write;

use heapless::String;

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
        let mut string: String<5> = String::new();
        write!(string, "G{:4}", telemetry.g_force.0).ok();
        buffer[..5].copy_from_slice(string.as_bytes());
        buffer[3] = to_number_with_dot(buffer[3], self.zero_dot);
        1
    }
}

#[cfg(test)]
mod test {
    use fixed_point::fixed;

    use super::GForce;
    use crate::drawable::Drawable;
    use crate::symbol::default_symbol_table;
    use crate::telemetry::Telemetry;
    use crate::test_utils::{to_utf8_string, ZeroSlice};

    #[test]
    fn test_g_force() {
        let mut buffer = [[0u8; 6]];
        let g_force = GForce::new(&default_symbol_table());
        let mut telemetry = Telemetry::default();
        telemetry.g_force = fixed!(1.1);
        g_force.draw(&telemetry, &mut buffer);
        assert_eq!("G  ⒈1 ", to_utf8_string(&buffer));

        buffer[0].zero();
        telemetry.g_force = fixed!(0.9);
        g_force.draw(&telemetry, &mut buffer);
        assert_eq!("G  ₀9 ", to_utf8_string(&buffer));
    }
}
