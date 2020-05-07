use numtoa::NumToA;

use crate::drawable::{Align, Drawable};
use crate::telemetry::Telemetry;

pub struct Height(Align); // Right only

impl Default for Height {
    fn default() -> Self {
        Self(Align::BottomRight)
    }
}

impl<T: AsMut<[u8]>> Drawable<T> for Height {
    fn align(&self) -> Align {
        self.0
    }

    fn draw(&self, telemetry: &Telemetry, output: &mut [T]) {
        if telemetry.height >= 1000 {
            return;
        }
        let mut buffer = output[0].as_mut();
        if self.0 == Align::BottomRight {
            buffer = output[output.len() - 1].as_mut();
        }
        let buffer_len = buffer.len();
        let region = &mut buffer[..buffer_len - 1];
        telemetry.height.numtoa(10, region);
        buffer[buffer_len - 1] = 'R' as u8;
    }
}

#[cfg(test)]
mod test {
    use crate::drawable::Drawable;
    use crate::telemetry::Telemetry;
    use crate::test_utils::to_utf8_string;

    use super::Height;

    #[test]
    fn test_height() {
        let mut buffer = [[0u8; 6]];
        let height = Height::default();
        let mut telemetry = Telemetry::default();
        telemetry.height = 998;
        height.draw(&telemetry, &mut buffer);
        assert_eq!("  998R", to_utf8_string(&buffer));

        buffer[0].iter_mut().for_each(|x| *x = 0);
        telemetry.height = 1000;
        height.draw(&telemetry, &mut buffer);
        assert_eq!("      ", to_utf8_string(&buffer));
    }
}
