use carlib::Car;

// We create a new module named `carlib`.
// If we don't do that, even private fields would become
// accessible from the outside.
mod carlib {

    // A trait is similar to an interface in Java or C#.
    pub trait Car {
        // I want these methods to manipulate the speed
        // of the car based on an acceleration property.
        fn accelerate(&mut self, duration: f64);
        fn brake(&mut self, force: f64);
        fn get_speed(&self) -> f64;
        fn get_name(&self) -> String;
        // The trait itself however cannot have any fields.
    }

    // So we define a struct that contains the fields
    // such as speed and acceleration.
    pub struct CarImpl {
        speed: f64,        // private field
        acceleration: f64, // private field
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
                name: name,
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
            // Not entirely sure whether I need this .clone().
            // In any case I do not want to return a reference
            // or even move it.
            self.speed.clone()
        }

        fn get_name(&self) -> String {
            self.name.clone()
        }
    }

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

        fn get_name(&self) -> String {
            self.imp.get_name()
        }
    }

    // Let's try a different approach.
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
}

// So this is just a little helper function to print the speed.
fn print_car_speed(car: &dyn carlib::Car) {
    // We can print name and speed via the trait.
    println!("{} travels at {} km/h", car.get_name(), car.get_speed());
    // However we cannot access the name field directly, since `car` is a trait
    // rather than a concrete object. So this would not compile:
    // println!("{} travels at {} km/h", car.name, car.get_speed());
}

fn print_concrete_car_speed(car: &carlib::CarImpl) {
    // We can now access the name field directly.
    // So this compiles fine:
    println!("{} travels at {} km/h", car.name, car.get_speed());
}

fn print_car(car: &Box<dyn carlib::Car>) {
    println!("{} travels at {} km/h", car.get_name(), car.get_speed());
}

fn main() {
    let mut car = carlib::CarImpl::new();
    // Uncommenting the following lines gives a compile error.
    // car.speed = 200.0; // speed is private
    car.name = "Carla".to_string(); // name is public and can be set.

    let mut other_car = carlib::CarImpl::new_named("Caroline".to_string());

    print_car_speed(&car);
    print_car_speed(&other_car);

    car.accelerate(100.0); // accelerate for 100 seconds

    print_car_speed(&car);
    print_concrete_car_speed(&other_car);

    other_car.accelerate(200.0);
    other_car.brake(0.2);

    print_car_speed(&car);
    print_car_speed(&other_car);

    // That is cool and all but what about the sports car?

    let mut flizzr = carlib::Sportscar::new_named("Flizzr".to_string());
    // Here the `imp` can not be accessed. It is private.
    //flizzr.imp.name = "Flizzr".to_string();
    flizzr.accelerate(100.0);
    print_car_speed(&flizzr);

    // let c = carlib::Car::new(); // This does not compile. Car is an interface.
    // let c = <dyn carlib::Car>::new(); // new() does not exist. So this does not compile either.

    let mut tuktuk = carlib::Familycar::new();
    tuktuk.imp.name = "Tuk-tuk".to_string();

    tuktuk.accelerate(100.0);
    print_car_speed(&tuktuk);

    // Now lets create a vector of cars.
    let mut cars: Vec<Box<dyn carlib::Car>> = Vec::new();
    cars.push(Box::new(carlib::Sportscar::new_named(
        "SpeedyMcSpeedface".to_string(),
    )));
    cars.push(Box::new(carlib::Familycar::new()));
    cars.push(Box::new(carlib::Sportscar::new()));

    // Iterate over all cars in the vector and accelerate them for 200 seconds.
    for car in cars.iter_mut() {
        car.accelerate(200.0);
        print_car(car);
    }
}
