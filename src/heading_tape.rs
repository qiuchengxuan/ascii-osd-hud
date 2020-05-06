use core::cell::Cell;
use core::cmp::min;

use numtoa::NumToA;

use crate::drawable::{Align, Drawable, Layer};
use crate::symbol;
use crate::telemetry::Telemetry;

const HEADING_TAPE_WIDTH: usize = 3 * 5; // e.g. "350 . 000 . 010"
const MAX_OFFSET: usize = HEADING_TAPE_WIDTH / 2;

#[inline]
fn degree_to_offset(degree: u16) -> usize {
    degree as usize * 3 / 5
}

fn theta_to_offset(theta: u16) -> usize {
    if theta < 180 {
        let offset = min(degree_to_offset(theta), MAX_OFFSET);
        MAX_OFFSET + offset
    } else {
        let offset = min(degree_to_offset(360 - theta), MAX_OFFSET);
        MAX_OFFSET - offset
    }
}

pub struct HeadingTape {
    align: Align, // only accept Top or Bottom
    sequence: usize,
    waypoint_indicator: symbol::Index,
    counter: Cell<usize>,
}

impl HeadingTape {
    fn new(waypoint_indicator: symbol::Index) -> HeadingTape {
        HeadingTape {
            align: Align::Top,
            sequence: 0,
            waypoint_indicator,
            counter: Cell::new(0),
        }
    }

    fn draw_indicator(&self, theta: u16, wp_theta: u16, output: &mut [u8]) {
        let index = output.len() / 2 - HEADING_TAPE_WIDTH / 2;
        let offset = theta_to_offset(theta) + index;
        let wp_offset = theta_to_offset(wp_theta) + index;
        if self.counter.get() % 2 == 0 || offset != wp_offset {
            output[offset] = '^' as u8;
        }
        if self.counter.get() % 2 == 1 || offset != wp_offset {
            output[wp_offset] = self.waypoint_indicator;
        }
    }
}

impl Drawable for HeadingTape {
    fn align(&self) -> Align {
        self.align
    }

    fn layer(&self) -> Layer {
        Layer::Top
    }

    fn draw<T: AsMut<[u8]>>(&self, telemetry: &Telemetry, output: &mut [T]) {
        let mut index = self.sequence;
        if self.align == Align::Bottom {
            index = output.len() - 1 - self.sequence;
        }
        draw_tape(telemetry.heading, output[index].as_mut());

        if self.align == Align::Top {
            index += 1
        } else {
            index -= 1
        };
        let wp_theta = telemetry.waypoint.coordinate.theta;
        self.draw_indicator(telemetry.attitude.yaw, wp_theta, output[index].as_mut());
        self.counter.set(self.counter.get() + 1)
    }
}

fn draw_heading(output: &mut [u8], number: u16) {
    let mut buffer: [u8; 5] = [0; 5];
    let num_str = number.numtoa(10, &mut buffer);
    output[..3].copy_from_slice(b"000");
    output[(3 - num_str.len())..3].copy_from_slice(num_str);
}

fn draw_tape(heading: u16, output: &mut [u8]) {
    let mut buffer: [u8; HEADING_TAPE_WIDTH + 4] = [0; HEADING_TAPE_WIDTH + 4];
    let lower_heading = heading / 10 * 10;
    let upper_heading = lower_heading + 10;
    let center = HEADING_TAPE_WIDTH / 2 + 2;
    let delta = degree_to_offset(heading - lower_heading);
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
    buffer[lower_index - 2] = '.' as u8;
    buffer[lower_index + 4] = '.' as u8;
    buffer[upper_index + 4] = '.' as u8;
    let index = output.len() / 2 - HEADING_TAPE_WIDTH / 2;
    output[index..index + HEADING_TAPE_WIDTH].copy_from_slice(&buffer[2..2 + HEADING_TAPE_WIDTH]);
}

#[cfg(test)]
mod test {
    use crate::drawable::Drawable;
    use crate::symbol::{Symbol, Symbols};
    use crate::telemetry::Telemetry;
    use crate::test_utils::to_utf8_string;

    use super::{HeadingTape, HEADING_TAPE_WIDTH};

    #[test]
    fn test_000_center_and_conflict_symbol() {
        let mut buffer: [[u8; HEADING_TAPE_WIDTH + 2]; 2] = [[0; HEADING_TAPE_WIDTH + 2]; 2];
        let tape = HeadingTape::new(Symbols::default().0[Symbol::BoxDrawningLightUp]);
        let telemetry = Telemetry::default();
        tape.draw(&telemetry, &mut buffer);
        assert_eq!(" 350 . 000 . 010 ", to_utf8_string(&buffer[0]));
        assert_eq!("        ^        ", to_utf8_string(&buffer[1]));
        tape.draw(&telemetry, &mut buffer);
        assert_eq!("        ╵        ", to_utf8_string(&buffer[1]));
    }

    #[test]
    fn test_different_heading() {
        let mut buffer: [[u8; HEADING_TAPE_WIDTH + 2]; 2] = [[0; HEADING_TAPE_WIDTH + 2]; 2];
        let tape = HeadingTape::new(Symbols::default().0[Symbol::BoxDrawningLightUp]);
        let mut telemetry = Telemetry::default();
        telemetry.heading = 359;
        tape.draw(&telemetry, &mut buffer);
        assert_eq!("  350 . 000 . 01 ", to_utf8_string(&buffer[0]));
        telemetry.heading = 358;
        tape.draw(&telemetry, &mut buffer);
        assert_eq!(" . 350 . 000 . 0 ", to_utf8_string(&buffer[0]));
        telemetry.heading = 356;
        tape.draw(&telemetry, &mut buffer);
        assert_eq!("  . 350 . 000 .  ", to_utf8_string(&buffer[0]));
    }

    #[test]
    fn test_yaw() {
        let mut buffer: [[u8; HEADING_TAPE_WIDTH + 2]; 2] = [[0; HEADING_TAPE_WIDTH + 2]; 2];
        let tape = HeadingTape::new(Symbols::default().0[Symbol::BoxDrawningLightUp]);
        let mut telemetry = Telemetry::default();
        telemetry.attitude.yaw = 358;
        tape.draw(&telemetry, &mut buffer);
        assert_eq!(" 350 . 000 . 010 ", to_utf8_string(&buffer[0]));
        assert_eq!("       ^╵        ", to_utf8_string(&buffer[1]));

        buffer[1].iter_mut().for_each(|x| *x = 0);
        telemetry.attitude.yaw = 300;
        tape.draw(&telemetry, &mut buffer);
        assert_eq!(" ^      ╵        ", to_utf8_string(&buffer[1]));

        buffer[1].iter_mut().for_each(|x| *x = 0);
        telemetry.attitude.yaw = 90;
        tape.draw(&telemetry, &mut buffer);
        assert_eq!("        ╵      ^ ", to_utf8_string(&buffer[1]));
    }

    #[test]
    fn test_waypoint() {
        let mut buffer: [[u8; HEADING_TAPE_WIDTH + 2]; 2] = [[0; HEADING_TAPE_WIDTH + 2]; 2];
        let tape = HeadingTape::new(Symbols::default().0[Symbol::BoxDrawningLightUp]);
        let mut telemetry = Telemetry::default();
        telemetry.waypoint.coordinate.theta = 90;
        tape.draw(&telemetry, &mut buffer);
        assert_eq!(" 350 . 000 . 010 ", to_utf8_string(&buffer[0]));
        assert_eq!("        ^      ╵ ", to_utf8_string(&buffer[1]));

        buffer[1].iter_mut().for_each(|x| *x = 0);
        telemetry.waypoint.coordinate.theta = 180;
        tape.draw(&telemetry, &mut buffer);
        assert_eq!(" ╵      ^        ", to_utf8_string(&buffer[1]));

        buffer[1].iter_mut().for_each(|x| *x = 0);
        telemetry.waypoint.coordinate.theta = 270;
        tape.draw(&telemetry, &mut buffer);
        assert_eq!(" ╵      ^        ", to_utf8_string(&buffer[1]));
    }
}
