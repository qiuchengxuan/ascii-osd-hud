use heapless::String;

use crate::drawable::{Align, Drawable, NumOfLine};
use crate::telemetry::Telemetry;

pub struct Vario(Align); // only accept TopRight or Right

impl Default for Vario {
    fn default() -> Self {
        Self(Align::Right)
    }
}

impl<T: AsMut<[u8]>> Drawable<T> for Vario {
    fn align(&self) -> Align {
        self.0
    }

    fn draw(&self, telemetry: &Telemetry, output: &mut [T]) -> NumOfLine {
        let buffer = output[0].as_mut();
        let string: String<6> = telemetry.vario.into();
        let bytes = string.as_bytes();
        let offset = buffer.len() - bytes.len();
        buffer[offset..].copy_from_slice(bytes);
        1
    }
}

#[cfg(test)]
mod test {
    use crate::drawable::Drawable;
    use crate::telemetry::Telemetry;
    use crate::test_utils::to_utf8_string;

    use super::Vario;

    #[test]
    fn test_altitude() {
        let mut buffer = [[0u8; 6]];
        let vario = Vario::default();
        let mut telemetry = Telemetry::default();
        telemetry.vario = 1000;
        vario.draw(&telemetry, &mut buffer);
        assert_eq!("  1000", to_utf8_string(&buffer));
        telemetry.vario = -1000;
        vario.draw(&telemetry, &mut buffer);
        assert_eq!(" -1000", to_utf8_string(&buffer));
    }
}
