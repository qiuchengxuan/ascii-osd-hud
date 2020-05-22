#![no_std]

#[derive(Copy, Clone, PartialEq)]
pub struct AspectRatio(pub i8, pub i8);

#[macro_export]
macro_rules! aspect_ratio {
    ($w:tt: $h:tt) => {
        AspectRatio($w, $h)
    };
}

impl AspectRatio {
    pub fn diagonal_to_width(&self, diagonal: usize) -> usize {
        match self {
            AspectRatio(16, 9) => diagonal * 1600 / 1835,
            AspectRatio(4, 3) => diagonal * 4 / 5,
            _ => diagonal * 1000 / 1414,
        }
    }

    pub fn diagonal_to_height(&self, diagonal: usize) -> usize {
        match self {
            AspectRatio(16, 9) => diagonal * 900 / 1835,
            AspectRatio(4, 3) => diagonal * 3 / 5,
            _ => diagonal * 1000 / 1414,
        }
    }
}

mod altitude;
mod aoa;
mod battery;
mod drawable;
mod flight_mode;
mod g_force;
mod heading_tape;
mod height;
pub mod hud;
mod pitch_ladder;
mod rssi;
mod speed;
pub mod symbol;
pub mod telemetry;
#[cfg(test)]
mod test_utils;
mod velocity_vector;
mod vertial_speed;
mod waypoint;
mod waypoint_vector;

extern crate micromath;

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(test)]
extern crate ascii;
