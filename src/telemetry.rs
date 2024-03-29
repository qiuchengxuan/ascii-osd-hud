use fixed_point::{fixed, FixedPoint};

#[derive(Copy, Clone, Debug)]
pub struct Attitude {
    pub roll: i16, // [-180, 180], clock wise
    pub pitch: i8, // [-90, 90], negative means sink
}

impl Default for Attitude {
    fn default() -> Self {
        Self { pitch: 0, roll: 0 }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SphericalCoordinate {
    pub rho: u16,   // ρ or radius
    pub theta: i16, // θ, -180 <= θ <= 180, azimuthal angle
    pub phi: i8,    // φ, -90 <= φ <= 90, polar angle, negative means desend
}

impl Default for SphericalCoordinate {
    fn default() -> Self {
        Self {
            rho: 0,
            theta: 0,
            phi: 0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Unit {
    Aviation,
    Metric,
}

impl Unit {
    pub fn distance(self) -> &'static str {
        match self {
            Self::Aviation => "NM",
            Self::Metric => "KM",
        }
    }

    pub fn elevation(self) -> &'static str {
        match self {
            Self::Aviation => "FT",
            Self::Metric => "M",
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Steerpoint<'a> {
    pub number: u8,                      // e.g. 0 means home or base
    pub name: &'a str,                   // e.g. "HOME" when number = 0
    pub heading: u16,                    //
    pub coordinate: SphericalCoordinate, // rho unit km or nm * 10
}

impl<'a> Default for Steerpoint<'a> {
    fn default() -> Self {
        Self {
            number: 0,
            name: "HOME",
            heading: 0,
            coordinate: SphericalCoordinate::default(),
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Notes<'a> {
    pub left: &'a str,
    pub center: &'a str,
    pub right: &'a str,
}

#[derive(Copy, Clone, Debug)]
pub struct Telemetry<'a> {
    pub altitude: i16,              // feets or meters
    pub aoa: FixedPoint<i8, 1>,     // in degree
    pub attitude: Attitude,         // in degree
    pub heading: u16,               // [0, 360), ref to north
    pub battery: u8,                // percentage
    pub g_force: FixedPoint<i8, 1>, // in g
    pub height: i16,                // feets or meters, same with altitude, i16::MIN means N/A
    pub notes: Notes<'a>,           //
    pub rssi: u8,                   // percentage
    pub unit: Unit,
    pub speed_vector: SphericalCoordinate, // rho unit km/h or knot, theta ref to attitude
    pub vario: i16,                        // feets/min or m/s
    pub steerpoint: Steerpoint<'a>,        //
}

impl<'a> Default for Telemetry<'a> {
    fn default() -> Telemetry<'a> {
        Telemetry {
            altitude: 0,
            attitude: Attitude::default(),
            heading: 0,
            aoa: fixed!(0.0),
            battery: 100,
            g_force: fixed!(1.0),
            height: 0,
            notes: Default::default(),
            rssi: 0,
            steerpoint: Steerpoint::default(),
            unit: Unit::Aviation,
            speed_vector: SphericalCoordinate::default(),
            vario: 0,
        }
    }
}

impl<'a> Telemetry<'a> {
    pub fn speed(&self) -> u16 {
        self.speed_vector.rho
    }

    pub fn time_to_go(&self) -> u32 {
        let rho = self.steerpoint.coordinate.rho;
        let speed = self.speed_vector.rho;
        if rho > 0 && speed > 0 {
            return rho as u32 * 3600 / 10 / speed as u32; // rho / 10 / (speed / 3600)
        }
        0
    }
}
