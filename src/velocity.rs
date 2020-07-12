use numtoa::NumToA;

use crate::drawable::{Align, Drawable, NumOfLine};
use crate::telemetry::Telemetry;

pub struct Velocity(Align); // only accept TopRight or Right

impl Default for Velocity {
    fn default() -> Self {
        Self(Align::Right)
    }
}

impl<T: AsMut<[u8]>> Drawable<T> for Velocity {
    fn align(&self) -> Align {
        self.0
    }

    fn draw(&self, telemetry: &Telemetry, output: &mut [T]) -> NumOfLine {
        let buffer = output[0].as_mut();
        let buffer_len = buffer.len();
        let velocity = telemetry.velocity;
        velocity.numtoa(10, &mut buffer[buffer_len - 6..]);
        1
    }
}

#[cfg(test)]
mod test {
    use crate::drawable::Drawable;
    use crate::telemetry::Telemetry;
    use crate::test_utils::to_utf8_string;

    use super::Velocity;

    #[test]
    fn test_altitude() {
        let mut buffer = [[0u8; 6]];
        let velocity = Velocity::default();
        let mut telemetry = Telemetry::default();
        telemetry.velocity = 1000;
        velocity.draw(&telemetry, &mut buffer);
        assert_eq!("  1000", to_utf8_string(&buffer));
        telemetry.velocity = -1000;
        velocity.draw(&telemetry, &mut buffer);
        assert_eq!(" -1000", to_utf8_string(&buffer));
    }
}
