pub struct Attitude {
    pub pitch: u16,
    pub roll: u16,
    pub yaw: u16, // ref to current heading
}

pub struct SphericalCoordinate {
    pub rho: u16,   // ρ or radius, dimensionless
    pub theta: u16, // θ, 0 <= θ < 360, azimuthal angle
    pub phi: u16,   // φ, 0 <= φ < 180, polar angle
}

pub struct Waypoint<'a> {
    pub coordinate: SphericalCoordinate,
    pub number: u8,    // e.g. 0 means home or base
    pub name: &'a str, // e.g. "HOME" when number = 0
}

pub struct Data<'a> {
    pub attitude: Attitude,  // in degree
    pub altitude: u16,       // dimensionless
    pub speed: u16,          // dimensionless
    pub vertical_speed: u16, // dimensionless
    pub aoa: u16,            // in degree*10
    pub heading: u16,        // in degree
    pub g_force: u8,         // in g*10
    pub flight_mode: &'a str,
    pub waypoint: Waypoint<'a>,
}

impl<'a> Default for Data<'a> {
    fn default() -> Data<'a> {
        Data {
            altitude: 3000,
            heading: 0,
            attitude: Attitude {
                pitch: 0,
                roll: 0,
                yaw: 0,
            },
            speed: 100,
            vertical_speed: 10,
            aoa: 30,
            g_force: 0,
            flight_mode: "MAN",
            waypoint: Waypoint {
                coordinate: SphericalCoordinate {
                    rho: 0,
                    theta: 0,
                    phi: 0,
                },
                number: 0,
                name: "HOME",
            },
        }
    }
}

pub trait DataSource<'a> {
    fn get(&self) -> Data<'a>;
}
