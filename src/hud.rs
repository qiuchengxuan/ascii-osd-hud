use enum_map::{enum_map, Enum, EnumMap};

use crate::altitude::Altitude;
use crate::aoa::AOA;
use crate::battery::Battery;
use crate::drawable::{Align, Drawable};
use crate::g_force::GForce;
use crate::heading_tape::HeadingTape;
use crate::height::Height;
use crate::note::note;
use crate::pitch_ladder::Pitchladder;
use crate::rssi::RSSI;
use crate::speed::Speed;
use crate::speed_vector::SpeedVector;
use crate::steerpoint::Steerpoint;
use crate::steerpoint_vector::SteerpointVector;
use crate::symbol::SymbolTable;
use crate::telemetry::Telemetry;
use crate::vario::Vario;
use crate::{AspectRatio, PixelRatio};

#[derive(Enum)]
pub enum Displayable {
    // Bottom
    Pitchladder,

    // Center
    SpeedVector,
    SteerpointVector,

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
    Vario,

    // BottomRight
    Steerpoint,
    Height,
}

pub struct HUD {
    altitude: Altitude,
    aoa: AOA,
    battery: Battery,
    g_force: GForce,
    heading_tape: HeadingTape,
    height: Height,
    pitch_ladder: Pitchladder,
    rssi: RSSI,
    speed: Speed,
    vario: Vario,
    speed_vector: SpeedVector,
    steerpoint: Steerpoint,
    steerpoint_vector: SteerpointVector,
    aligns: EnumMap<Displayable, Option<Align>>,
}

impl HUD {
    pub fn new(symbols: &SymbolTable, fov: u8, pixel: PixelRatio, aspect: AspectRatio) -> Self {
        let fov = core::cmp::max(10, fov); // avoid divide zero
        HUD {
            altitude: Altitude::default(),
            aoa: AOA::new(&symbols),
            battery: Battery::new(&symbols),
            g_force: GForce::new(&symbols),
            heading_tape: HeadingTape::new(&symbols),
            height: Height::default(),
            pitch_ladder: Pitchladder::new(&symbols, fov, pixel, aspect),
            rssi: RSSI::new(&symbols),
            speed: Speed::default(),
            vario: Vario::default(),
            speed_vector: SpeedVector::new(&symbols, fov, aspect),
            steerpoint_vector: SteerpointVector::new(&symbols, fov, aspect),
            steerpoint: Steerpoint::new(&symbols),
            aligns: enum_map! {
                Displayable::Altitude => Some(Align::Right),
                Displayable::AOA => Some(Align::Left),
                Displayable::Battery => Some(Align::TopRight),
                Displayable::GForce => Some(Align::Left),
                Displayable::HeadingTape => Some(Align::Top),
                Displayable::Height => Some(Align::Bottom),
                Displayable::Pitchladder => Some(Align::Center),
                Displayable::RSSI => Some(Align::TopLeft),
                Displayable::Speed => Some(Align::Left),
                Displayable::Vario => Some(Align::Right),
                Displayable::SpeedVector => Some(Align::Center),
                Displayable::Steerpoint => Some(Align::BottomRight),
                Displayable::SteerpointVector => Some(Align::Center),
            },
        }
    }

    fn to_drawable<'b, B: AsMut<[u8]>>(&'b self, displayable: Displayable) -> &'b dyn Drawable<B> {
        match displayable {
            Displayable::Altitude => &self.altitude,
            Displayable::AOA => &self.aoa,
            Displayable::Battery => &self.battery,
            Displayable::GForce => &self.g_force,
            Displayable::HeadingTape => &self.heading_tape,
            Displayable::Height => &self.height,
            Displayable::Pitchladder => &self.pitch_ladder,
            Displayable::RSSI => &self.rssi,
            Displayable::Speed => &self.speed,
            Displayable::Vario => &self.vario,
            Displayable::SpeedVector => &self.speed_vector,
            Displayable::Steerpoint => &self.steerpoint,
            Displayable::SteerpointVector => &self.steerpoint_vector,
        }
    }

    pub fn draw<'b, B: AsMut<[u8]>>(
        &self,
        telemetry: &Telemetry<'b>,
        output: &'b mut [B],
    ) -> &'b [B] {
        output.iter_mut().for_each(|line| {
            for x in line.as_mut() {
                if *x == b' ' {
                    *x = 0
                } else if *x > 0 {
                    *x = b' '
                }
            }
        });
        let output_len = output.len();
        let mut indexes: EnumMap<Align, usize> = EnumMap::new();
        for (display, align_option) in self.aligns.iter() {
            let align = match align_option {
                Some(align) => *align,
                None => continue,
            };
            let drawable: &dyn Drawable<B> = self.to_drawable(display);
            let region = match align {
                Align::Top | Align::TopLeft | Align::TopRight => &mut output[indexes[align]..],
                Align::Bottom | Align::BottomLeft | Align::BottomRight => {
                    #[cfg(test)]
                    println!("{}", indexes[align]);
                    &mut output[..output_len - indexes[align]]
                }
                Align::Left | Align::Right => &mut output[output_len / 2 + indexes[align]..],
                _ => output,
            };
            indexes[align] += drawable.draw(&telemetry, region);
        }

        indexes[Align::Center] = 2;
        let region = &mut output[output_len / 2 + indexes[Align::Left]..];
        indexes[Align::Left] += note(telemetry.notes.left, Align::Left, region);
        let region = &mut output[output_len / 2 + indexes[Align::Center]..];
        indexes[Align::Center] += note(telemetry.notes.center, Align::Center, region);
        let region = &mut output[output_len / 2 + indexes[Align::Right]..];
        indexes[Align::Right] += note(telemetry.notes.right, Align::Right, region);
        output
    }
}

#[cfg(test)]
mod test {
    use crate::symbol::default_symbol_table;
    use crate::telemetry::{Attitude, Notes, SphericalCoordinate, Steerpoint, Telemetry};
    use crate::test_utils::{fill_edge, to_utf8_string};
    use crate::{AspectRatio, PixelRatio};

    use super::HUD;

    fn default_telemetry() -> Telemetry<'static> {
        Telemetry {
            altitude: 1000,
            attitude: Attitude {
                pitch: 10,
                roll: 10,
            },
            heading: 10,
            aoa: 31,
            g_force: 11,
            height: 99,
            notes: Notes {
                left: "MAN",
                ..Default::default()
            },
            rssi: 100,
            vario: 100,
            speed_vector: SphericalCoordinate {
                rho: 100, // speed
                theta: 10,
                phi: -5,
            },
            steerpoint: Steerpoint {
                coordinate: SphericalCoordinate {
                    rho: 47,
                    theta: -10,
                    phi: -14,
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[test]
    fn test_hud() {
        let mut buffer = [[0u8; 30]; 16];
        let symbols = default_symbol_table();
        let px_ratio = pixel_ratio!(16:30);
        let hud = HUD::new(&symbols, 150, px_ratio, aspect_ratio!(16:9));
        let telemetry = default_telemetry();
        hud.draw(&telemetry, &mut buffer);
        fill_edge(&mut buffer);

        let expected = "⏉100    000 . 010 . 020   β100\
                        .        ╵     ^             .\
                        .                            .\
                        .                            .\
                        .                            .\
                        ▔⎺⎺⎻⎻─⎼⎼⎽⎽▁                  .\
                        .          ▔▔⎺⎺⎻──⎼⎼⎽▁▁      .\
                        .                      ▔▔⎺⎻⎻──\
                        . 100                     1000\
                        ⍺  ⒊1            ⏂         100\
                        G  ⒈1                        .\
                        MAN          ☐               .\
                        .                            .\
                        .                       0/HOME\
                        .                         ⒋7NM\
                        .             99      00:02:49";
        assert_eq!(expected, to_utf8_string(&buffer));
    }
}
