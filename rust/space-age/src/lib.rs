// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    value: f64,
}

const MERCURY_ORBITAL_PERIOD: f64 = 0.2408467;
const VENUS_ORBITAL_PERIOD: f64 = 0.61519726;
const EARTH_ORBITAL_PERIOD: f64 = 1.0;
const MARS_ORBITAL_PERIOD: f64 = 1.8808158;
const JUPITER_ORBITAL_PERIOD: f64 = 11.862615;
const SATURN_ORBITAL_PERIOD: f64 = 29.447498;
const URANUS_ORBITAL_PERIOD: f64 = 84.016846;
const NEPTUNE_ORBITAL_PERIOD: f64 = 164.79132;

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        let s = s as f64;
        const EARTH_DAYS_IN_A_YEAR: f64 = 365.25_f64;
        const EARTH_HOURS_IN_A_DAY: f64 = 24_f64;
        const MINUTES_IN_AN_HOUR: f64 = 60_f64;
        const SECONDS_IN_AN_HOUR: f64 = 60_f64;
        Self { value: s / (EARTH_DAYS_IN_A_YEAR * EARTH_HOURS_IN_A_DAY * MINUTES_IN_AN_HOUR * SECONDS_IN_AN_HOUR) }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! create_planet {
    ($planet:ident, $period:ident) => {
        pub struct $planet;
        impl Planet for $planet {
            fn years_during(d: &Duration) -> f64 {
                d.value / $period
            }
        }
    };
}

create_planet!(Mercury, MERCURY_ORBITAL_PERIOD);
create_planet!(Venus, VENUS_ORBITAL_PERIOD);
create_planet!(Earth, EARTH_ORBITAL_PERIOD);
create_planet!(Mars, MARS_ORBITAL_PERIOD);
create_planet!(Jupiter, JUPITER_ORBITAL_PERIOD);
create_planet!(Saturn, SATURN_ORBITAL_PERIOD);
create_planet!(Uranus, URANUS_ORBITAL_PERIOD);
create_planet!(Neptune, NEPTUNE_ORBITAL_PERIOD);
