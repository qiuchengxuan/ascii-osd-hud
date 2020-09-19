use core::cell::Cell;
use core::cmp::{max, min};

use heapless::consts::U5;
use heapless::String;

use crate::drawable::{Align, Drawable, NumOfLine};
use crate::symbol::{Symbol, SymbolIndex, SymbolTable};
use crate::telemetry::Telemetry;

const HEADING_TAPE_WIDTH: usize = 3 * 5; // e.g. "350 . 000 . 010"
const MAX_OFFSET: isize = HEADING_TAPE_WIDTH as isize / 2;

#[inline]
fn degree_to_offset(degree: i16) -> isize {
    degree as isize * 3 / 5
}

fn theta_to_offset(theta: i16) -> usize {
    let offset = if theta >= 0 {
        min(degree_to_offset(theta), MAX_OFFSET)
    } else {
        max(degree_to_offset(theta), -MAX_OFFSET)
    };
    (MAX_OFFSET + offset) as usize
}

pub struct HeadingTape {
    align: Align, // only accept Top or Bottom
    steerpoint_indicator: SymbolIndex,
    counter: Cell<usize>,
}

impl HeadingTape {
    pub fn new(symbols: &SymbolTable) -> Self {
        Self {
            align: Align::Top,
            steerpoint_indicator: symbols[Symbol::BoxDrawningLightUp],
            counter: Cell::new(0),
        }
    }

    fn draw_indicator(&self, wp_theta: i16, output: &mut [u8]) {
        let center = output.len() / 2;
        let wp_offset = theta_to_offset(wp_theta) + center - HEADING_TAPE_WIDTH / 2;
        if self.counter.get() % 2 == 0 || wp_offset != center {
            output[center] = b'^';
        }
        if self.counter.get() % 2 == 1 || wp_offset != center {
            output[wp_offset] = self.steerpoint_indicator;
        }
    }
}

impl<T: AsMut<[u8]>> Drawable<T> for HeadingTape {
    fn align(&self) -> Align {
        self.align
    }

    fn draw(&self, telemetry: &Telemetry, output: &mut [T]) -> NumOfLine {
        let mut index = 0;
        if self.align == Align::Bottom {
            index = output.len() - 1;
        }
        draw_tape(telemetry.heading, output[index].as_mut());

        if self.align == Align::Top {
            index += 1
        } else {
            index -= 1
        };
        let mut theta = ((telemetry.steerpoint.heading + 360 - telemetry.heading) % 360) as i16;
        if theta > 180 {
            theta = theta - 360
        }
        self.draw_indicator(theta, output[index].as_mut());
        self.counter.set(self.counter.get() + 1);
        2
    }
}

fn draw_heading(output: &mut [u8], heading: u16) {
    output[..3].copy_from_slice(b"000");
    let string: String<U5> = heading.into();
    let bytes = string.as_bytes();
    output[(3 - bytes.len())..3].copy_from_slice(bytes);
}

fn draw_tape(heading: u16, output: &mut [u8]) {
    let mut buffer: [u8; HEADING_TAPE_WIDTH + 4] = [b' '; HEADING_TAPE_WIDTH + 4];
    let lower_heading = heading / 10 * 10;
    let upper_heading = lower_heading + 10;
    let center = HEADING_TAPE_WIDTH / 2 + 2;
    let delta = degree_to_offset((heading - lower_heading) as i16);
    let lower_index = center - 1 - delta as usize;

    draw_heading(&mut buffer[lower_index..], lower_heading);

    let upper_index = lower_index + 6;
    draw_heading(&mut buffer[upper_index..], upper_heading % 360);

    if lower_index >= 6 {
        let heading = (lower_heading + 360 - 10) % 360;
        draw_heading(&mut buffer[lower_index - 6..], heading);
    }
    if upper_index + 6 < 2 + HEADING_TAPE_WIDTH {
        draw_heading(&mut buffer[upper_index + 6..], (upper_heading + 10) % 360)
    }
    buffer[lower_index - 2] = b'.';
    buffer[lower_index + 4] = b'.';
    buffer[upper_index + 4] = b'.';
    let index = output.len() / 2 - HEADING_TAPE_WIDTH / 2;
    output[index..index + HEADING_TAPE_WIDTH].copy_from_slice(&buffer[2..2 + HEADING_TAPE_WIDTH]);
}

#[cfg(test)]
mod test {
    use crate::drawable::Drawable;
    use crate::symbol::default_symbol_table;
    use crate::telemetry::Telemetry;
    use crate::test_utils::{to_utf8_string, ZeroSlice};

    use super::{HeadingTape, HEADING_TAPE_WIDTH};

    #[test]
    fn test_000_center_and_conflict_symbol() {
        let mut buffer: [[u8; HEADING_TAPE_WIDTH + 2]; 2] = [[0; HEADING_TAPE_WIDTH + 2]; 2];
        let tape = HeadingTape::new(&default_symbol_table());
        let telemetry = Telemetry::default();
        tape.draw(&telemetry, &mut buffer);
        assert_eq!(" 350 . 000 . 010 ", to_utf8_string(&buffer[0..1]));
        assert_eq!("        ^        ", to_utf8_string(&buffer[1..2]));
        tape.draw(&telemetry, &mut buffer);
        assert_eq!("        ╵        ", to_utf8_string(&buffer[1..2]));
    }

    #[test]
    fn test_different_heading() {
        let mut buffer: [[u8; HEADING_TAPE_WIDTH + 2]; 2] = [[0; HEADING_TAPE_WIDTH + 2]; 2];
        let tape = HeadingTape::new(&default_symbol_table());
        let mut telemetry = Telemetry::default();
        telemetry.heading = 359;
        tape.draw(&telemetry, &mut buffer);
        assert_eq!("  350 . 000 . 01 ", to_utf8_string(&buffer[0..1]));
        telemetry.heading = 358;
        tape.draw(&telemetry, &mut buffer);
        assert_eq!(" . 350 . 000 . 0 ", to_utf8_string(&buffer[0..1]));
        telemetry.heading = 356;
        tape.draw(&telemetry, &mut buffer);
        assert_eq!("  . 350 . 000 .  ", to_utf8_string(&buffer[0..1]));
    }

    #[test]
    fn test_steerpoint() {
        let mut buffer = [[0u8; HEADING_TAPE_WIDTH + 2]; 2];
        let tape = HeadingTape::new(&default_symbol_table());
        let mut telemetry = Telemetry::default();
        telemetry.steerpoint.heading = 0;
        tape.draw(&telemetry, &mut buffer);
        assert_eq!(" 350 . 000 . 010 ", to_utf8_string(&buffer[0..1]));
        assert_eq!("        ^        ", to_utf8_string(&buffer[1..2]));

        buffer[1].zero();
        telemetry.steerpoint.heading = 90;
        tape.draw(&telemetry, &mut buffer);
        assert_eq!("        ^      ╵ ", to_utf8_string(&buffer[1..2]));

        buffer[1].zero();
        telemetry.steerpoint.heading = 180;
        tape.draw(&telemetry, &mut buffer);
        assert_eq!("        ^      ╵ ", to_utf8_string(&buffer[1..2]));

        buffer[1].zero();
        telemetry.steerpoint.heading = 270;
        tape.draw(&telemetry, &mut buffer);
        assert_eq!(" ╵      ^        ", to_utf8_string(&buffer[1..2]));
    }
}
