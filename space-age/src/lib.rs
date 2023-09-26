// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: u64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
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

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        let earth_years = (d.seconds as f64) / 31557600f64;
        earth_years / 0.2408467f64
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        let earth_years = (d.seconds as f64) / 31557600f64;
        earth_years / 0.61519726f64
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        (d.seconds as f64) / 31557600f64
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        let earth_years = (d.seconds as f64) / 31557600f64;
        earth_years / 1.8808158f64
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        let earth_years = (d.seconds as f64) / 31557600f64;
        earth_years / 11.862615f64
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        let earth_years = (d.seconds as f64) / 31557600f64;
        earth_years / 29.447498f64
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        let earth_years = (d.seconds as f64) / 31557600f64;
        earth_years / 84.016846f64
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        let earth_years = (d.seconds as f64) / 31557600f64;
        earth_years / 164.79132f64
    }
}
