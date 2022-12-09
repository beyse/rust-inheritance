extend_basic_car!(
    Familycar,
    (), //Familycar don't need to add any field to the basic car
    fn get_acceleration(&self) -> f64 {
        Familycar::ACCELERATION
    }
);
impl Familycar {
    const ACCELERATION: f64 = 0.05; // Quite slow.

    pub fn new(name: Option<String>) -> Self {
        Self {
            speed: 0.0,
            name,
            extended: (),
        }
    }
}
