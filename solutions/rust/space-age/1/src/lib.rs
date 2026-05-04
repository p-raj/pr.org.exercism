// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: f64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration::from_secs(s)
    }
}

impl Duration {
    fn from_secs(seconds: u64) -> Self {
        Duration{seconds: seconds as f64}
    }
    
    fn as_secs(self: &Duration) -> f64 {
        self.seconds
        
    }
}

const EARTH_ORB_PERIOD: f64 = 1.0;
const EARTH_YEAR_IN_SEC: f64 = 365.25*24.0*60.0*60.0*EARTH_ORB_PERIOD;

pub trait Planet {
    fn round(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }
    fn years_during(d: &Duration) -> f64 {
        Self::round(d.as_secs()/EARTH_YEAR_IN_SEC)
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

const MERCURY_ORB_PERIOD: f64 = 0.2408467;
impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        Earth::years_during(d) * (EARTH_ORB_PERIOD/MERCURY_ORB_PERIOD)
    }
}

const VENUS_ORB_PERIOD: f64 = 0.61519726;
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        Earth::years_during(d) * (EARTH_ORB_PERIOD/VENUS_ORB_PERIOD)
    }
}
impl Planet for Earth {}

const MARS_ORB_PERIOD: f64 = 1.8808158;
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        Earth::years_during(d) * (EARTH_ORB_PERIOD/MARS_ORB_PERIOD)
    }
}

const JUPITER_ORB_PERIOD: f64 = 11.862615;
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        Earth::years_during(d) * (EARTH_ORB_PERIOD/JUPITER_ORB_PERIOD)
    }
}

const SATURN_ORB_PERIOD: f64 = 29.447498;
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        Earth::years_during(d) * (EARTH_ORB_PERIOD/SATURN_ORB_PERIOD)
    }
}

const URANUS_ORB_PERIOD: f64 = 84.016846;
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        Earth::years_during(d) * (EARTH_ORB_PERIOD/URANUS_ORB_PERIOD)
    }
}

const NEPTURE_ORB_PERIOD: f64 = 164.79132;
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        Earth::years_during(d) * (EARTH_ORB_PERIOD/NEPTURE_ORB_PERIOD)
    }
}