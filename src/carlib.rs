//NOTE: macro need to came before `pub mod`. Could be fixed if we organize this
//crate as an lib instead of a bin.
//
//Create an extension of "abstract class" basic_car. You can see that like an
//version of this.
//```
//abstract struct BasicCar {
//  speed: f64,
//  name: Option<String>,
//  pub fn get_speed(&self) -> f64 { ... }
//  fn set_speed(&mut self, new_speed: f64) { ... }
//  pub fn get_name(&self) -> &str { ... }
//  pub fn accelerate(&mut self, duration: f64) { ... }
//  pub fn brake(&mut self, force: f64) { ... }
//  fn get_acceleration();
//}
//```
//
//The basic car only have the speed and name fields, and only require the
//get_acceleration function to be implemented.
macro_rules! extend_basic_car {
    ($name:ident, $extend:ty, $get_acceleration:item $(,)?) => {
        #[allow(dead_code)]
        pub struct $name {
            speed: f64,
            name: Option<String>,
            extended: $extend,

        }
        impl crate::carlib::inner::InnerCar for $name {
            fn get_speed(&self) -> f64 {
                self.speed
            }
            fn set_speed(&mut self, new_speed: f64) {
                self.speed = new_speed;
            }
            fn get_name(&self) -> &str {
                if let Some(name) = &self.name {
                    &name
                } else {
                    "<unnamed>"
                }
            }
            $get_acceleration
        }
    };
}

pub mod family_car;
pub mod sports_car;

//private stuff, family/sports car can access this, but not main
mod inner {
    use super::Car;

    pub trait InnerCar {
        fn get_speed(&self) -> f64;
        fn set_speed(&mut self, new_speed: f64);
        fn get_name(&self) -> &str;
        fn get_acceleration(&self) -> f64;
        fn accelerate(&mut self, duration: f64) {
            // We can access the private fields of the struct
            let new_speed =
                self.get_speed() + (self.get_acceleration() * duration);
            self.set_speed(new_speed);
        }

        fn brake(&mut self, force: f64) {
            let mut new_speed =
                self.get_speed() - (force * self.get_acceleration());
            if new_speed < 0.0 {
                // My cars will never go backwards.
                // Take that, physics!
                new_speed = 0.0;
            }
            self.set_speed(new_speed);
        }
    }

    //impl Car for all "Objects" that also implement InnerCar by
    //forcing all calls to be redirect to the InnerCar implementation.
    impl<T: InnerCar> Car for T {
        //NOTE: if we simply use `self.accelerate(duration)` we will be calling
        //the Car implementation of the function, making this an infinite
        //recursion.
        fn accelerate(&mut self, duration: f64) {
            <Self as InnerCar>::accelerate(self, duration)
        }

        fn brake(&mut self, force: f64) {
            <Self as InnerCar>::brake(self, force)
        }

        fn get_speed(&self) -> f64 {
            <Self as InnerCar>::get_speed(self)
        }

        fn get_name(&self) -> &str {
            <Self as InnerCar>::get_name(self)
        }
    }
}

// A trait is similar to an interface in Java or C#.
pub trait Car {
    // I want these methods to manipulate the speed
    // of the car based on an acceleration property.
    fn accelerate(&mut self, duration: f64);
    fn brake(&mut self, force: f64);
    fn get_speed(&self) -> f64;
    fn get_name(&self) -> &str;
    // The trait itself however cannot have any fields.
}
