#[derive(Copy, Clone, Debug)]
pub struct Attitude {
    pub pitch: i8, // [-90, 90], negative means sink
    pub roll: i16, // [-180, 180], clock wise
}

impl Default for Attitude {
    fn default() -> Self {
        Self { pitch: 0, roll: 0 }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SphericalCoordinate {
    pub rho: u16,   // ρ or radius
    pub theta: u16, // θ, 0 <= θ < 360, azimuthal angle
    pub phi: i16,   // φ, -90 <= φ <= 90, polar angle, negative means desend
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
pub struct Waypoint {
    pub number: u8,                      // e.g. 0 means home or base
    pub name: [u8; 4],                   // e.g. "HOME" when number = 0
    pub coordinate: SphericalCoordinate, // rho unit km or nm * 10
    pub unit: [u8; 2],                   // KM or NM
}

impl Default for Waypoint {
    fn default() -> Self {
        Self {
            number: 0,
            name: *b"HOME",
            coordinate: SphericalCoordinate::default(),
            unit: *b"NM",
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Telemetry {
    pub altitude: i16,                        // feets or meters
    pub aoa: u8,                              // in degree*10
    pub attitude: Attitude,                   // in degree
    pub heading: u16,                         // [0, 360), ref to north
    pub battery: u8,                          // percentage
    pub flight_mode: [u8; 4],                 //
    pub g_force: u8,                          // in g*10
    pub height: i16,                          // feets or meters, same with altitude
    pub rssi: u8,                             // percentage
    pub velocity_vector: SphericalCoordinate, // rho unit km/h or knot, theta ref to attitude
    pub vertical_speed: i16,                  // feets/min or m/s
    pub waypoint: Waypoint,                   //
}

impl<'a> Default for Telemetry {
    fn default() -> Telemetry {
        Telemetry {
            altitude: 0,
            attitude: Attitude::default(),
            heading: 0,
            aoa: 0,
            battery: 100,
            flight_mode: *b"MAN ",
            g_force: 0,
            height: 0,
            rssi: 0,
            velocity_vector: SphericalCoordinate::default(),
            vertical_speed: 0,
            waypoint: Waypoint::default(),
        }
    }
}

impl Telemetry {
    pub fn speed(&self) -> u16 {
        self.velocity_vector.rho
    }

    pub fn time_to_go(&self) -> u32 {
        let rho = self.waypoint.coordinate.rho;
        let speed = self.velocity_vector.rho;
        if rho > 0 && speed > 0 {
            return rho as u32 * 3600 / 10 / speed as u32; // rho / 10 / (speed / 3600)
        }
        0
    }
}

pub trait TelemetrySource {
    fn get_telemetry(&self) -> Telemetry;
}
