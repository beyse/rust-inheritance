use crate::carlib::car::Car;
use crate::carlib::car_concept::CarConcept;
use crate::carlib::car_impl::CarImpl;

// Rust seems to encourage composition over direct inheritance.
// I like that. 👍
pub struct Sportscar {
    // A Sportcar is a class that has an implementation
    // `imp` of a car called `CarImpl`. Makes sense.
    imp: CarImpl, //Perhaps there is a better name than imp?
}

// Here is the implementation of my sportscar.
impl Sportscar {
    pub fn new() -> Sportscar {
        let mut obj = Sportscar {
            imp: CarImpl::new(),
        };
        obj.imp.acceleration = 0.84; // I like fast cars and .84 is double of .42.
        obj
    }
    pub fn new_named(name: String) -> Sportscar {
        let mut obj = Sportscar {
            imp: CarImpl::new_named(name),
        };
        // I don't quite like the duplication here.
        obj.imp.acceleration = 0.84;
        obj
    }
}

// Here we implement the Car trait for the Sportscar.
// Easy enough boilerplate code. But quite some boilerplate still.
impl Car for Sportscar {
    fn accelerate(&mut self, duration: f64) {
        self.imp.accelerate(duration);
    }

    fn brake(&mut self, force: f64) {
        self.imp.brake(force);
    }

    fn get_speed(&self) -> f64 {
        self.imp.get_speed()
    }

    fn get_name(&self) -> &str {
        self.imp.get_name()
    }
}

// This is an alternative way for polymorphism. We used
// composition to get the car behavior into the sports
// car so now we can return it as a car.
// In a real application you should chose one way or the other.
impl CarConcept for Sportscar {
    fn as_car(&self) -> &dyn Car {
        &self.imp
    }
    fn as_mut_car(&mut self) -> &mut dyn Car {
        &mut self.imp
    }
}
