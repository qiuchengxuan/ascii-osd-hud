#![no_std]

mod altitude;
mod aoa;
mod data_source;
mod drawable;
mod heading;
mod hud;
mod symbol;
#[cfg(test)]
mod test_utils;

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(test)]
extern crate ascii;
