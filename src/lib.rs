#![no_std]

#[derive(Copy, Clone, PartialEq)]
pub struct PixelRatio(pub u8, pub u8);

impl From<(u8, u8)> for PixelRatio {
    fn from(tuple: (u8, u8)) -> Self {
        Self(tuple.0, tuple.1)
    }
}

#[macro_export]
macro_rules! pixel_ratio {
    ($w:tt: $h:tt) => {
        PixelRatio($w, $h)
    };
}

#[derive(Copy, Clone, PartialEq)]
pub struct AspectRatio(pub u8, pub u8);

impl From<(u8, u8)> for AspectRatio {
    fn from(tuple: (u8, u8)) -> Self {
        Self(tuple.0, tuple.1)
    }
}

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
mod steerpoint;
mod steerpoint_vector;
pub mod symbol;
pub mod telemetry;
#[cfg(test)]
mod test_utils;
mod velocity;
mod velocity_vector;

extern crate micromath;

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(test)]
extern crate ascii;
