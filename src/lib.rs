#![no_std]

#[derive(Copy, Clone, PartialEq)]
pub enum AspectRatio {
    Standard, // 4:3
    Wide,     // 16:9
}

impl AspectRatio {
    pub fn diagonal_to_width(&self, diagonal: usize) -> usize {
        match self {
            AspectRatio::Wide => diagonal * 1600 / 1835,
            AspectRatio::Standard => diagonal * 4 / 5,
        }
    }

    pub fn diagonal_to_height(&self, diagonal: usize) -> usize {
        match self {
            AspectRatio::Wide => diagonal * 900 / 1835,
            AspectRatio::Standard => diagonal * 3 / 5,
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

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(test)]
extern crate ascii;
