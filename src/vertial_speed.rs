use numtoa::NumToA;

use crate::drawable::{Align, Drawable};
use crate::telemetry::Telemetry;

pub struct VerticalSpeed(Align); // only accept TopRight or Right

impl Default for VerticalSpeed {
    fn default() -> Self {
        Self(Align::Right)
    }
}

impl<T: AsMut<[u8]>> Drawable<T> for VerticalSpeed {
    fn align(&self) -> Align {
        self.0
    }

    fn draw(&self, telemetry: &Telemetry, output: &mut [T]) {
        let buffer = output[0].as_mut();
        let buffer_len = buffer.len();
        telemetry
            .vertical_speed
            .numtoa(10, &mut buffer[buffer_len - 6..]);
    }
}

#[cfg(test)]
mod test {
    use crate::drawable::Drawable;
    use crate::telemetry::Telemetry;
    use crate::test_utils::to_utf8_string;

    use super::VerticalSpeed;

    #[test]
    fn test_altitude() {
        let mut buffer = [[0u8; 6]];
        let vertical_speed = VerticalSpeed::default();
        let mut telemetry = Telemetry::default();
        telemetry.vertical_speed = 1000;
        vertical_speed.draw(&telemetry, &mut buffer);
        assert_eq!("  1000", to_utf8_string(&buffer));
        telemetry.vertical_speed = -1000;
        vertical_speed.draw(&telemetry, &mut buffer);
        assert_eq!(" -1000", to_utf8_string(&buffer));
    }
}
