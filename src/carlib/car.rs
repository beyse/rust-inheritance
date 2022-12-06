// We create a new module named `carlib`.
// If we don't do that, even private fields would become
// accessible from the outside.

// A trait is similar to an interface in Java or C#.
pub const UNNAMED_CAR: &str = "<unnamed>";
pub trait Car {
    // I want these methods to manipulate the speed
    // of the car based on an acceleration property.
    fn get_acceleration(&self) -> f64;
    fn get_speed(&self) -> f64;
    fn set_speed(&mut self, new_speed: f64);
    fn get_name(&self) -> &str;
    fn accelerate(&mut self, duration: f64) {
        let speed = self.get_speed() + (self.get_acceleration() * duration);
        self.set_speed(speed);
    }
    fn brake(&mut self, force: f64) {
        let mut speed = self.get_speed() - (force * self.get_speed());
        if speed < 0.0 {
            // My cars will never go backwards.
            // Take that, physics!
            speed = 0.0;
        }
        self.set_speed(speed);
    }
    // The trait itself however cannot have any fields.
}
