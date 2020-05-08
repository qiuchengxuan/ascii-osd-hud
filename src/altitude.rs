use numtoa::NumToA;

use crate::drawable::{Align, Drawable, NumOfLine};
use crate::telemetry::Telemetry;

pub struct Altitude(Align); // only accept TopRight or Right

impl Default for Altitude {
    fn default() -> Self {
        Self(Align::Right)
    }
}

impl<T: AsMut<[u8]>> Drawable<T> for Altitude {
    fn align(&self) -> Align {
        self.0
    }

    fn draw(&self, telemetry: &Telemetry, output: &mut [T]) -> NumOfLine {
        telemetry.altitude.numtoa(10, output[0].as_mut());
        1
    }
}

#[cfg(test)]
mod test {
    use crate::drawable::Drawable;
    use crate::telemetry::Telemetry;
    use crate::test_utils::to_utf8_string;

    use super::Altitude;

    #[test]
    fn test_altitude() {
        let mut buffer = [[0u8; 6]];
        let altitude = Altitude::default();
        let mut telemetry = Telemetry::default();
        telemetry.altitude = 3000;
        altitude.draw(&telemetry, &mut buffer);
        assert_eq!("  3000", to_utf8_string(&buffer));
        telemetry.altitude = 30000;
        altitude.draw(&telemetry, &mut buffer);
        assert_eq!(" 30000", to_utf8_string(&buffer));
    }
}
