use core::cell::Cell;

use enum_map::Enum;

use crate::telemetry::TelemetrySource;

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
    telemetry_source: &'a dyn TelemetrySource<'a>,
    fps: u8,
    counter: Cell<u8>,
    fov: u8,
}

impl<'a> HUD<'a> {
    pub fn new(source: &'a dyn TelemetrySource<'a>, fps: u8) -> HUD<'a> {
        HUD {
            telemetry_source: source,
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

    pub fn draw(&self, output: &[&mut [u8]]) {
        let _telemetry = self.telemetry_source.get_telemetry();
    }
}
