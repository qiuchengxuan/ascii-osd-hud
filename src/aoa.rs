use numtoa::NumToA;

use crate::drawable::{Align, Drawable, Layer};
use crate::symbol;
use crate::telemetry::Telemetry;

pub struct AOA {
    alpha: symbol::Index,
    zero_dot: symbol::Index,
    sequence: usize,
}

impl AOA {
    fn new(alpha: symbol::Index, zero_dot: symbol::Index, sequence: usize) -> AOA {
        AOA {
            alpha,
            zero_dot,
            sequence,
        }
    }
}

impl Drawable for AOA {
    fn align(&self) -> Align {
        Align::Left
    }

    fn layer(&self) -> Layer {
        Layer::Top
    }

    fn draw<T: AsMut<[u8]>>(&self, telemetry: &Telemetry, output: &mut [T]) {
        let mut num_buffer: [u8; 5] = [0; 5];
        let num_str = telemetry.aoa.numtoa(10, &mut num_buffer);
        let buffer = output[output.len() / 2 + self.sequence].as_mut();
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
    use crate::symbol::{Symbol, Symbols};
    use crate::telemetry::Telemetry;
    use crate::test_utils::to_utf8_string;

    use super::AOA;

    #[test]
    fn test_aoa() {
        let mut buffer: [[u8; 4]; 1] = [[0; 4]];
        let symbols = Symbols::default();
        let aoa = AOA::new(
            symbols.0[Symbol::Alpha],
            symbols.0[Symbol::ZeroWithTraillingDot],
            0,
        );
        let mut telemetry = Telemetry::default();
        telemetry.aoa = 31;
        aoa.draw(&telemetry, &mut buffer);
        assert_eq!("⍺ ₃1", to_utf8_string(&buffer[0]));
    }
}
