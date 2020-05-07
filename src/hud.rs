use enum_map::{enum_map, Enum, EnumMap};

use crate::altitude::Altitude;
use crate::aoa::AOA;
use crate::battery::Battery;
use crate::drawable::{Align, Drawable};
use crate::flight_mode::FlightMode;
use crate::g_force::GForce;
use crate::heading_tape::HeadingTape;
use crate::height::Height;
use crate::rssi::RSSI;
use crate::speed::Speed;
use crate::symbol::SymbolTable;
use crate::telemetry::TelemetrySource;
use crate::vertial_speed::VerticalSpeed;

#[derive(Enum)]
pub enum Displayable {
    // TopLeft
    RSSI,

    // Top
    HeadingTape,

    // TopRight,
    Battery,

    // Left
    Speed,
    AOA,
    GForce,

    // Right
    Altitude,
    VerticalSpeed,

    // BottomLeft
    FlightMode,

    // BottomRight
    Height,
}

pub struct HUD<'a> {
    altitude: Altitude,
    aoa: AOA,
    battery: Battery,
    flight_mode: FlightMode,
    g_force: GForce,
    heading_tape: HeadingTape,
    height: Height,
    rssi: RSSI,
    speed: Speed,
    vertial_speed: VerticalSpeed,
    aligns: EnumMap<Displayable, Option<Align>>,
    telemetry_source: &'a dyn TelemetrySource,
}

impl<'a> HUD<'a> {
    pub fn new(source: &'a dyn TelemetrySource, symbols: &'a SymbolTable) -> HUD<'a> {
        HUD {
            altitude: Altitude::default(),
            aoa: AOA::new(&symbols),
            battery: Battery::new(&symbols),
            flight_mode: FlightMode::default(),
            g_force: GForce::new(&symbols),
            heading_tape: HeadingTape::new(&symbols),
            height: Height::default(),
            rssi: RSSI::new(&symbols),
            speed: Speed::default(),
            vertial_speed: VerticalSpeed::default(),
            aligns: enum_map! {
                Displayable::Altitude => Some(Align::Right),
                Displayable::AOA => Some(Align::Left),
                Displayable::Battery => Some(Align::TopRight),
                Displayable::FlightMode => Some(Align::BottomLeft),
                Displayable::GForce => Some(Align::Left),
                Displayable::HeadingTape => Some(Align::Top),
                Displayable::Height => Some(Align::BottomRight),
                Displayable::RSSI => Some(Align::TopLeft),
                Displayable::Speed => Some(Align::Left),
                Displayable::VerticalSpeed => Some(Align::Right),
            },
            telemetry_source: source,
        }
    }

    fn to_drawable<'b, T: AsMut<[u8]>>(&'b self, displayable: Displayable) -> &'b dyn Drawable<T> {
        match displayable {
            Displayable::Altitude => &self.altitude,
            Displayable::AOA => &self.aoa,
            Displayable::Battery => &self.battery,
            Displayable::FlightMode => &self.flight_mode,
            Displayable::GForce => &self.g_force,
            Displayable::HeadingTape => &self.heading_tape,
            Displayable::Height => &self.height,
            Displayable::RSSI => &self.rssi,
            Displayable::Speed => &self.speed,
            Displayable::VerticalSpeed => &self.vertial_speed,
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
    use crate::telemetry::{Attitude, Telemetry, TelemetrySource};
    use crate::test_utils::to_utf8_string;

    use super::HUD;

    struct StubTelemetrySource;

    impl TelemetrySource for StubTelemetrySource {
        fn get_telemetry(&self) -> Telemetry {
            Telemetry {
                altitude: 3000,
                attitude: Attitude {
                    yaw: 357,
                    ..Default::default()
                },
                aoa: 31,
                g_force: 11,
                heading: 90,
                height: 999,
                rssi: 100,
                speed: 100,
                vertical_speed: 100,
                ..Default::default()
            }
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
                line[line.len() - 1] = '.' as u8;
            }
            if *line.first().unwrap() == 0u8 {
                line[0] = '.' as u8;
            }
        });
        let expected = "⏉100    080 . 090 . 100   β100\
                        .             ^╵             .\
                        .                            .\
                        .                            .\
                        .                            .\
                        .                            .\
                        .                            .\
                        .                            .\
                        . 100                     3000\
                        ⍺  ₃1                      100\
                        g  ₁1                        .\
                        .                            .\
                        .                            .\
                        .                            .\
                        .                            .\
                        MAN                       999R";
        assert_eq!(expected, to_utf8_string(&buffer));
    }
}
