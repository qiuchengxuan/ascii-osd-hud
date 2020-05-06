use numtoa::NumToA;

use crate::drawable::{Align, Drawable};
use crate::telemetry::Telemetry;

pub struct Speed(Align); // only accept TopLeft or Left

impl Default for Speed {
    fn default() -> Speed {
        Speed(Align::Left)
    }
}

impl<T: AsMut<[u8]>> Drawable<T> for Speed {
    fn align(&self) -> Align {
        self.0
    }

    fn draw(&self, telemetry: &Telemetry, output: &mut [T]) {
        telemetry.speed.numtoa(10, &mut output[0].as_mut()[..5]);
    }
}

#[cfg(test)]
mod test {
    use super::Speed;
    use crate::drawable::Drawable;
    use crate::telemetry::Telemetry;
    use crate::test_utils::to_utf8_string;

    #[test]
    fn test_altitude() {
        let mut buffer = [[0u8; 6]];
        let altitude = Speed::default();
        let telemetry = Telemetry::default();
        altitude.draw(&telemetry, &mut buffer);
        assert_eq!("  100 ", to_utf8_string(&buffer));
    }
}
