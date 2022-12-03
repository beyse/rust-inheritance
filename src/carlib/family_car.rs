use crate::carlib::car::Car;
use crate::carlib::car_impl::CarImpl;

// Let's try a different approach with the family car.
pub struct Familycar {
    pub imp: CarImpl,
}

impl Familycar {
    const ACCELERATION: f64 = 0.05; // Quite slow.

    pub fn new() -> Familycar {
        let mut obj = Familycar {
            imp: CarImpl::new(),
        };
        obj.imp.acceleration = Familycar::ACCELERATION;
        return obj;
    }
}

// Here we implement the Car trait for the Familycar.
// This time we use the `delegate!` macro.
// With this macro we can get rid of quite some boilerplate code.
impl Car for Familycar {
    delegate::delegate! {
        to self.imp {
            fn accelerate(&mut self, duration: f64);
            fn brake(&mut self, force: f64);
            fn get_speed(&self) -> f64;
            fn get_name(&self) -> String;
        }
    }
}
