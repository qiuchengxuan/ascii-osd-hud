use core::cell::Cell;

use enum_map::Enum;

use crate::data_source::DataSource;

#[derive(Enum)]
pub enum DisplayType {
    Speed,
    Altitude,
    HeadingTape,
    GForce,
    AOA,
    Battery,
    FlightMode,
    Waypoint,
    FlightPathLadder,
    VelocityVector,
    WaypointVector,
}

pub enum AspectRatio {
    Standard,
    Wide,
}

pub struct HUD<'a> {
    data_source: &'a dyn DataSource<'a>,
    fps: u8,
    counter: Cell<u8>,
    fov: u8,
}

impl<'a> HUD<'a> {
    pub fn new(source: &'a dyn DataSource<'a>, fps: u8) -> HUD<'a> {
        HUD {
            data_source: source,
            fps,
            counter: Cell::new(0),
            fov: 150,
        }
    }

    pub fn set_fov(&mut self, fov: u8) {
        if fov > 0 {
            self.fov = fov;
        }
    }

    pub fn dump(&self, output: &[&mut [u8]]) {
        // for entry in DisplayType::iter() {}
        // self.counter.set(self.counter.get() + 1)
    }
}
