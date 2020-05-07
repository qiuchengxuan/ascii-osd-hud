pub struct Attitude {
    pub pitch: u16,
    pub roll: u16,
    pub yaw: u16, // ref to current heading
}

impl Default for Attitude {
    fn default() -> Self {
        Self {
            pitch: 0,
            roll: 0,
            yaw: 0,
        }
    }
}

pub struct SphericalCoordinate {
    pub rho: u16,   // ρ or radius, dimensionless
    pub theta: u16, // θ, 0 <= θ < 360, azimuthal angle
    pub phi: u16,   // φ, 0 <= φ < 180, polar angle
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

pub struct Waypoint {
    pub coordinate: SphericalCoordinate,
    pub number: u8,    // e.g. 0 means home or base
    pub name: [u8; 4], // e.g. "HOME" when number = 0
}

impl Default for Waypoint {
    fn default() -> Self {
        Self {
            coordinate: SphericalCoordinate::default(),
            number: 0,
            name: *b"HOME",
        }
    }
}

pub struct Telemetry {
    pub altitude: u16,        // dimensionless
    pub aoa: u8,              // in degree*10
    pub attitude: Attitude,   // in degree
    pub battery: u8,          // percentage
    pub flight_mode: [u8; 4], // or AP mode
    pub g_force: u8,          // in g*10
    pub heading: u16,         // in degree
    pub height: u16,          // dimensionless
    pub rssi: u8,             // percentage
    pub speed: u16,           // dimensionless
    pub vertical_speed: i16,  // dimensionless
    pub waypoint: Waypoint,
}

impl<'a> Default for Telemetry {
    fn default() -> Telemetry {
        Telemetry {
            altitude: 0,
            attitude: Attitude::default(),
            aoa: 0,
            battery: 100,
            flight_mode: *b"MAN ",
            g_force: 0,
            heading: 0,
            height: 0,
            rssi: 0,
            speed: 0,
            vertical_speed: 0,
            waypoint: Waypoint::default(),
        }
    }
}

pub trait TelemetrySource {
    fn get_telemetry(&self) -> Telemetry;
}
