use numtoa::NumToA;

use crate::drawable::{Align, Drawable, NumOfLine};
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

    fn draw(&self, telemetry: &Telemetry, output: &mut [T]) -> NumOfLine {
        if telemetry.height >= 100 {
            return 0;
        }
        let mut buffer = output[0].as_mut();
        if self.0 == Align::BottomRight {
            buffer = output[output.len() - 1].as_mut();
        }
        let buffer_len = buffer.len();
        let region = &mut buffer[..buffer_len - 1];
        telemetry.height.numtoa(10, region);
        buffer[buffer_len - 1] = 'R' as u8;
        1
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
        let mut buffer = [[0u8; 7]];
        let height = Height::default();
        let mut telemetry = Telemetry::default();
        telemetry.height = 98;
        height.draw(&telemetry, &mut buffer);
        assert_eq!("    98R", to_utf8_string(&buffer));

        buffer[0].iter_mut().for_each(|x| *x = 0);
        telemetry.height = 100;
        height.draw(&telemetry, &mut buffer);
        assert_eq!("       ", to_utf8_string(&buffer));
    }
}
