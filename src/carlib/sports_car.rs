use crate::carlib::car::{Car, CarInheritance};
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
        return obj;
    }
    pub fn new_named(name: String) -> Sportscar {
        let mut obj = Sportscar {
            imp: CarImpl::new_named(name),
        };
        // I don't quite like the duplication here.
        obj.imp.acceleration = 0.84;
        return obj;
    }
}
impl CarInheritance for Sportscar {
    fn as_car(&self) -> &dyn Car {
        &self.imp
    }
    fn as_mut_car(&mut self) -> &mut dyn Car {
        &mut self.imp
    }
}
