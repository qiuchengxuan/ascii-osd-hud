#![no_std]

mod altitude;
mod aoa;
mod drawable;
mod heading_tape;
mod hud;
mod symbol;
mod telemetry;
#[cfg(test)]
mod test_utils;

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(test)]
extern crate ascii;
