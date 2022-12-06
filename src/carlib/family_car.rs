use crate::carlib::car::Car;

// Let's try a different approach with the family car.
const ACCELERATION: f64 = 0.05; // Quite slow.
pub struct Familycar {
    //don't need acceleration to be a variable, because it never changes
    pub name: String,
    pub speed: f64,
}

impl Familycar {
    pub fn new(name: Option<String>) -> Self {
        Self {
            name: name.unwrap_or("<unnamed>".to_string()),
            speed: 0.0,
        }
    }
}

// Here we implement the Car trait for the Familycar.
// This time we use the `delegate!` macro.
// With this macro we can get rid of quite some boilerplate code.
impl Car for Familycar {
    fn get_speed(&self) -> f64 {
        self.speed
    }
    fn set_speed(&mut self, new_speed: f64) {
        self.speed = new_speed;
    }
    fn get_acceleration(&self) -> f64 {
        ACCELERATION
    }
    fn get_name(&self) -> &str {
        &self.name
    }
}
