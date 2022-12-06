use crate::carlib::car::Car;

use super::car::UNNAMED_CAR;

// Rust seems to encourage composition over direct inheritance.
// I like that. 👍
const DEFAULT_ACCELERATION: f64 = 0.84; // I like fast cars and .84 is double of .42.
pub struct Sportscar {
    pub acceleration: f64,
    pub name: String,
    pub speed: f64,
}

// Here is the implementation of my sportscar.
impl Sportscar {
    pub fn new(name: Option<String>, acceleration: Option<f64>) -> Self {
        Self {
            name: name.unwrap_or(UNNAMED_CAR.to_string()),
            acceleration: acceleration.unwrap_or(DEFAULT_ACCELERATION),
            speed: 0.0,
        }
    }
}

// Here we implement the Car trait for the Sportscar.
// Easy enough boilerplate code. But quite some boilerplate still.
impl Car for Sportscar {
    fn get_speed(&self) -> f64 {
        self.speed
    }
    fn set_speed(&mut self, new_speed: f64) {
        self.speed = new_speed;
    }
    fn get_acceleration(&self) -> f64 {
        self.acceleration
    }
    fn get_name(&self) -> &str {
        &self.name
    }
}
