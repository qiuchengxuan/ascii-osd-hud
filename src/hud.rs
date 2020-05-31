use enum_map::{enum_map, Enum, EnumMap};

use crate::altitude::Altitude;
use crate::aoa::AOA;
use crate::battery::Battery;
use crate::drawable::{Align, Drawable};
use crate::flight_mode::FlightMode;
use crate::g_force::GForce;
use crate::heading_tape::HeadingTape;
use crate::height::Height;
use crate::pitch_ladder::Pitchladder;
use crate::rssi::RSSI;
use crate::speed::Speed;
use crate::symbol::SymbolTable;
use crate::telemetry::TelemetrySource;
use crate::velocity_vector::VelocityVector;
use crate::vertial_speed::VerticalSpeed;
use crate::waypoint::Waypoint;
use crate::waypoint_vector::WaypointVector;
use crate::AspectRatio;

#[derive(Enum)]
pub enum Displayable {
    // Bottom
    Pitchladder,
    VelocityVector,
    WaypointVector,

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
    FlightMode,

    // Right
    Altitude,
    VerticalSpeed,

    // BottomRight
    Waypoint,
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
    pitch_ladder: Pitchladder,
    rssi: RSSI,
    speed: Speed,
    vertial_speed: VerticalSpeed,
    velocity_vector: VelocityVector,
    waypoint: Waypoint,
    waypoint_vector: WaypointVector,
    aligns: EnumMap<Displayable, Option<Align>>,
    telemetry_source: &'a dyn TelemetrySource,
}

impl<'a> HUD<'a> {
    pub fn new(
        source: &'a dyn TelemetrySource,
        symbols: &SymbolTable,
        fov: u8,
        aspect_ratio: AspectRatio,
    ) -> HUD<'a> {
        HUD {
            altitude: Altitude::default(),
            aoa: AOA::new(&symbols),
            battery: Battery::new(&symbols),
            flight_mode: FlightMode::default(),
            g_force: GForce::new(&symbols),
            heading_tape: HeadingTape::new(&symbols),
            height: Height::default(),
            pitch_ladder: Pitchladder::new(&symbols, fov, aspect_ratio),
            rssi: RSSI::new(&symbols),
            speed: Speed::default(),
            vertial_speed: VerticalSpeed::default(),
            velocity_vector: VelocityVector::new(&symbols, fov, aspect_ratio),
            waypoint_vector: WaypointVector::new(&symbols, fov, aspect_ratio),
            waypoint: Waypoint::new(&symbols),
            aligns: enum_map! {
                Displayable::Altitude => Some(Align::Right),
                Displayable::AOA => Some(Align::Left),
                Displayable::Battery => Some(Align::TopRight),
                Displayable::FlightMode => Some(Align::Left),
                Displayable::GForce => Some(Align::Left),
                Displayable::HeadingTape => Some(Align::Top),
                Displayable::Height => Some(Align::BottomRight),
                Displayable::Pitchladder => Some(Align::Center),
                Displayable::RSSI => Some(Align::TopLeft),
                Displayable::Speed => Some(Align::Left),
                Displayable::VerticalSpeed => Some(Align::Right),
                Displayable::VelocityVector => Some(Align::Center),
                Displayable::Waypoint => Some(Align::BottomRight),
                Displayable::WaypointVector => Some(Align::Center),
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
            Displayable::Pitchladder => &self.pitch_ladder,
            Displayable::RSSI => &self.rssi,
            Displayable::Speed => &self.speed,
            Displayable::VerticalSpeed => &self.vertial_speed,
            Displayable::VelocityVector => &self.velocity_vector,
            Displayable::Waypoint => &self.waypoint,
            Displayable::WaypointVector => &self.waypoint_vector,
        }
    }

    pub fn draw<'b, T: AsMut<[u8]>>(&self, output: &'b mut [T]) -> &'b [T] {
        output.iter_mut().for_each(|line| {
            for x in line.as_mut() {
                if *x == ' ' as u8 {
                    *x = 0
                } else if *x > 0 {
                    *x = ' ' as u8
                }
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
            indexes[align] += drawable.draw(&telemetry, region);
        }
        output
    }
}

#[cfg(test)]
mod test {
    use crate::symbol::default_symbol_table;
    use crate::telemetry::{Attitude, SphericalCoordinate, Telemetry, TelemetrySource, Waypoint};
    use crate::test_utils::{fill_edge, to_utf8_string};
    use crate::AspectRatio;

    use super::HUD;

    struct StubTelemetrySource;

    impl TelemetrySource for StubTelemetrySource {
        fn get_telemetry(&self) -> Telemetry {
            Telemetry {
                altitude: 1000,
                attitude: Attitude {
                    pitch: 10,
                    roll: 10,
                    yaw: 10, // heading
                    ..Default::default()
                },
                aoa: 31,
                g_force: 11,
                height: 99,
                rssi: 100,
                vertical_speed: 100,
                velocity_vector: SphericalCoordinate {
                    rho: 100, // speed
                    theta: 10,
                    phi: -5,
                },
                waypoint: Waypoint {
                    coordinate: SphericalCoordinate {
                        rho: 47,
                        theta: 350,
                        phi: -14,
                    },
                    ..Default::default()
                },
                ..Default::default()
            }
        }
    }

    #[test]
    fn test_hud() {
        let mut buffer = [[0u8; 30]; 16];
        let symbols = default_symbol_table();
        let hud = HUD::new(&StubTelemetrySource {}, &symbols, 150, aspect_ratio!(16:9));
        hud.draw(&mut buffer);
        fill_edge(&mut buffer);

        let expected = "⏉100    000 . 010 . 020   β100\
                        .        ╵     ^             .\
                        .                            .\
                        .                            .\
                        .                            .\
                        .                            .\
                        .                            .\
                        .                            .\
                        . 100                     1000\
                        ⍺  ⒊1⎽▁▁         ⏂         100\
                        g  ⒈1   ▔▔⎺⎻⎻──⎼⎽⎽▁▁         .\
                        MAN          ☐      ▔⎺⎺⎻⎻─⎼⎼⎽⎽\
                        .                          99R\
                        .                       0/HOME\
                        .                         ⒋7NM\
                        .                     00:02:49";
        assert_eq!(expected, to_utf8_string(&buffer));
    }
}
