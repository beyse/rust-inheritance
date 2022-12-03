// We create a new module named `carlib`.
// If we don't do that, even private fields would become
// accessible from the outside.

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

//trait to make easy for an "object" to return a refernece to itself that impl
//Car
pub trait CarInheritance {
    fn as_car(&self) -> &dyn Car;
    fn as_mut_car(&mut self) -> &mut dyn Car;
}
