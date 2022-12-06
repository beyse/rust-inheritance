// Import the Car trait
use crate::carlib::car::Car;

// So we define a struct that contains the fields
// such as speed and acceleration.
pub struct CarImpl {
    speed: f64,            // private field
    pub acceleration: f64, // public field
    // A car can be named anything anytime
    // So no physics applies. Thus I make it public.
    pub name: String, // public field
}

// If we do not want to expose CarImpl to the outside,
// we would write:
// ```
// struct CarImpl { ... }
// ```
// without the `pub` keyword.

// Here we implement the constructor for the struct.
impl CarImpl {
    pub fn new() -> CarImpl {
        CarImpl {
            speed: 0.0,
            acceleration: 0.1,
            // By default any car is unnamed.
            name: String::from("<unnamed>"),
        }
    }
    // An alternative constructor that allows to set
    // the name of the car uppon construction.
    pub fn new_named(name: String) -> CarImpl {
        CarImpl {
            speed: 0.0,
            acceleration: 0.1,
            name, // We can just use the shorthand here.
        }
    }
}

// Here we implement the Car trait (interface) for the struct.
impl Car for CarImpl {
    // My physics for acceleration may be wrong here
    // but it's just a toy example.

    fn accelerate(&mut self, duration: f64) {
        // We can access the private fields of the struct
        self.speed += self.acceleration * duration;
    }

    fn brake(&mut self, force: f64) {
        self.speed -= force * self.speed;
        if self.speed < 0.0 {
            // My cars will never go backwards.
            // Take that, physics!
            self.speed = 0.0;
        }
    }

    fn get_speed(&self) -> f64 {
        self.speed
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}
