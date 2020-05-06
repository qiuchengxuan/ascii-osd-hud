use enum_map::{enum_map, Enum, EnumMap};

use crate::altitude::Altitude;
use crate::aoa::AOA;
use crate::drawable::{Align, Drawable};
use crate::heading_tape::HeadingTape;
use crate::speed::Speed;
use crate::symbol::SymbolTable;
use crate::telemetry::TelemetrySource;

#[derive(Enum)]
pub enum Displayable {
    Speed,
    AOA,
    Altitude,
    HeadingTape,
}

pub struct HUD<'a> {
    speed: Speed,
    aoa: AOA,
    altitude: Altitude,
    heading_tape: HeadingTape,
    aligns: EnumMap<Displayable, Option<Align>>,
    telemetry_source: &'a dyn TelemetrySource<'a>,
}

impl<'a> HUD<'a> {
    pub fn new(source: &'a dyn TelemetrySource<'a>, symbols: &'a SymbolTable) -> HUD<'a> {
        HUD {
            speed: Speed::default(),
            aoa: AOA::new(&symbols),
            altitude: Altitude::default(),
            heading_tape: HeadingTape::new(&symbols),
            aligns: enum_map! {
                Displayable::Speed => Some(Align::Left),
                Displayable::AOA => Some(Align::Left),
                Displayable::Altitude => Some(Align::Right),
                Displayable::HeadingTape => Some(Align::Top),
            },
            telemetry_source: source,
        }
    }

    fn to_drawable<'b, T: AsMut<[u8]>>(&'b self, displayable: Displayable) -> &'b dyn Drawable<T> {
        match displayable {
            Displayable::Speed => &self.speed,
            Displayable::AOA => &self.aoa,
            Displayable::Altitude => &self.altitude,
            Displayable::HeadingTape => &self.heading_tape,
        }
    }

    pub fn draw<T: AsMut<[u8]>>(&self, output: &mut [T]) {
        output.iter_mut().for_each(|line| {
            for x in line.as_mut() {
                *x = 0
            }
        });
        let output_len = output.len();
        let mut indexes: EnumMap<Align, usize> = EnumMap::new();
        let telemetry = self.telemetry_source.get_telemetry();
        for (display, align_option) in self.aligns.iter() {
            if align_option.is_none() {
                continue;
            }
            let align = align_option.unwrap();
            let drawable: &dyn Drawable<T> = self.to_drawable(display);
            let region = match align {
                Align::Top | Align::TopLeft | Align::TopRight => &mut output[indexes[align]..],
                Align::Bottom | Align::BottomLeft | Align::BottomRight => {
                    &mut output[..output_len - indexes[align]]
                }
                Align::Left | Align::Right => &mut output[output_len / 2 + indexes[align]..],
                _ => output,
            };
            drawable.draw(&telemetry, region);
            indexes[align] += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::symbol::default_symbol_table;
    use crate::telemetry::{Telemetry, TelemetrySource};
    use crate::test_utils::to_utf8_string;

    use super::HUD;

    struct StubTelemetrySource;

    impl<'a> TelemetrySource<'a> for StubTelemetrySource {
        fn get_telemetry(&self) -> Telemetry<'a> {
            Telemetry::default()
        }
    }

    #[test]
    fn test_hud() {
        let mut buffer = [[0u8; 30]; 16];
        let symbols = default_symbol_table();
        let hud = HUD::new(&StubTelemetrySource {}, &symbols);
        hud.draw(&mut buffer);
        buffer.iter_mut().for_each(|mutable| {
            let line = mutable.as_mut();
            if *line.last().unwrap() == 0u8 {
                line[line.len() - 1] = '$' as u8;
            }
            if *line.first().unwrap() == 0u8 {
                line[0] = '~' as u8;
            }
        });
        let expected = "~       350 . 000 . 010      $\
                        ~              ^             $\
                        ~                            $\
                        ~                            $\
                        ~                            $\
                        ~                            $\
                        ~                            $\
                        ~                            $\
                        ~ 100                     3000\
                        ⍺ ₃0                         $\
                        ~                            $\
                        ~                            $\
                        ~                            $\
                        ~                            $\
                        ~                            $\
                        ~                            $\
                        ";
        assert_eq!(expected, to_utf8_string(&buffer));
    }
}
