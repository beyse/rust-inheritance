use crate::carlib::car::{Car, CarInheritance};
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

impl CarInheritance for Familycar {
    fn as_car(&self) -> &dyn Car {
        &self.imp
    }
    fn as_mut_car(&mut self) -> &mut dyn Car {
        &mut self.imp
    }
}
