use numtoa::NumToA;

use crate::data_source::Data;
use crate::drawable::{Align, Drawable, Layer};

pub struct Altitude {
    align: Align, // only accept TopRight or Right
    sequence: usize,
}

impl Altitude {
    fn new(sequence: usize) -> Altitude {
        Altitude {
            align: Align::Right,
            sequence,
        }
    }
}

impl Drawable for Altitude {
    fn align(&self) -> Align {
        self.align
    }

    fn layer(&self) -> Layer {
        Layer::Top
    }

    fn draw<T: AsMut<[u8]>>(&self, data: &Data, output: &mut [T]) {
        let mut index = 0;
        if self.align == Align::Right {
            index = output.len() / 2;
        }
        let buffer = output[index + self.sequence].as_mut();
        let buffer_len = buffer.len();
        data.altitude.numtoa(10, &mut buffer[buffer_len - 5..]);
    }
}

#[cfg(test)]
mod test {
    use crate::data_source::Data;
    use crate::drawable::Drawable;
    use crate::test_utils::to_utf8_string;

    use super::Altitude;

    #[test]
    fn test_aoa() {
        let mut buffer: [[u8; 6]; 1] = [[0; 6]];
        let altitude = Altitude::new(0);
        let mut data = Data::default();
        altitude.draw(&data, &mut buffer);
        assert_eq!("  3000", to_utf8_string(&buffer[0]));
        data.altitude = 30000;
        altitude.draw(&data, &mut buffer);
        assert_eq!(" 30000", to_utf8_string(&buffer[0]));
    }
}
