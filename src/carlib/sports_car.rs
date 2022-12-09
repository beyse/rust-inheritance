//SportsCar just need to extend the basic car with the acceleration field,
//that should be defined on the object creation.
struct InnerSportsCar {
    acceleration: f64,
}

extend_basic_car!(
    Sportscar,
    InnerSportsCar,
    fn get_acceleration(&self) -> f64 {
        self.extended.acceleration
    }
);

impl Sportscar {
    const DEFAULT_ACCELERATION: f64 = 0.84;
    pub fn new(name: Option<String>, acceleration: Option<f64>) -> Self {
        Self {
            speed: 0.0,
            name,
            extended: InnerSportsCar {
                acceleration: acceleration
                    .unwrap_or(Self::DEFAULT_ACCELERATION),
            },
        }
    }
}
