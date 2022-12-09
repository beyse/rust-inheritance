mod carlib;
use crate::carlib::family_car::Familycar;
use crate::carlib::sports_car::Sportscar;
use crate::carlib::Car;

// So this is just a little helper function to print the speed.
fn print_car_speed(car: &dyn Car) {
    // We can print name and speed via the trait.
    println!("{} travels at {} km/h", car.get_name(), car.get_speed());
    // However we cannot access the name field directly, since `car` is a trait
    // rather than a concrete object. So this would not compile:
    // println!("{} travels at {} km/h", car.name, car.get_speed());
}

// fn print_concrete_car_speed(car: &carlib::CarImpl) {
//     // We can now access the name field directly.
//     // So this compiles fine:
//     println!("{} travels at {} km/h", car.name, car.get_speed());
// }

// We could use the dyn keyword to make the function accept any Car.
// fn print_car(car: &Box<dyn Car>) {
//     println!("{} travels at {} km/h", car.get_name(), car.get_speed());
// }

// Or we use a generic that is more encouraged by Rust.
fn print_car(car: &dyn Car) {
    println!("{} travels at {} km/h", car.get_name(), car.get_speed());
}

fn main() {
    // The below code is not working because the CarImpl is private.
    // To make is accessible one would need to edit the carlib.rs file:
    // ```rust
    // - mod car_impl;
    // + pub mod car_impl;
    // ```

    // let mut car = carlib::CarImpl::new();
    // // Uncommenting the following lines gives a compile error.
    // // car.speed = 200.0; // speed is private
    // car.name = "Carla".to_string(); // name is public and can be set.

    // let mut other_car = carlib::CarImpl::new_named("Caroline".to_string());

    // print_car_speed(&car);
    // print_car_speed(&other_car);

    // car.accelerate(100.0); // accelerate for 100 seconds

    // print_car_speed(&car);
    // print_concrete_car_speed(&other_car);

    // other_car.accelerate(200.0);
    // other_car.brake(0.2);

    // print_car_speed(&car);
    // print_car_speed(&other_car);

    // That is cool and all but what about the sports car?

    let mut flizzr = Sportscar::new(Some("Flizzr".to_string()), None);
    // Here the `imp` can not be accessed. It is private.
    //flizzr.imp.name = "Flizzr".to_string();
    flizzr.accelerate(100.0);
    print_car_speed(&flizzr);

    // let c = carlib::Car::new(); // This does not compile. Car is an interface.
    // let c = <dyn carlib::Car>::new(); // new() does not exist. So this does not compile either.

    let mut tuktuk = Familycar::new(Some("Tuk-tuk".to_string()));
    tuktuk.accelerate(100.0);
    print_car_speed(&tuktuk);

    // Now lets create a vector of cars.
    {
        // Scope so I can use cars again.
        let mut cars: Vec<Box<dyn Car>> = Vec::new();
        cars.push(Box::new(Sportscar::new(
            Some("SpeedyMcSpeedface".to_string()),
            Some(0.99), //faster then the default
        )));
        cars.push(Box::new(Familycar::new(None)));
        cars.push(Box::new(Sportscar::new(None, None)));

        // Iterate over all cars in the vector and accelerate them for 200 seconds.
        for car in cars.iter_mut() {
            car.accelerate(200.0);
            print_car(car.as_mut());
        }
    }

    // Alternatively we could also create a vector of CarConcepts.
    {
        let mut cars: Vec<Box<dyn Car>> = Vec::new();
        cars.push(Box::new(Sportscar::new(
            Some("FastyO'Fastboi".to_string()),
            None,
        )));
        cars.push(Box::new(Familycar::new(None)));
        cars.push(Box::new(Sportscar::new(None, None)));

        for car in cars.iter_mut() {
            car.accelerate(200.0);
            print_car(car.as_ref());
        }
    }
}
