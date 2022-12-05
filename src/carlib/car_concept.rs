use crate::Car;

// The car concept is a trait that lets you return a
// refernece to itself as a car even if it is no car.
pub trait CarConcept {
    fn as_car(&self) -> &dyn Car;
    fn as_mut_car(&mut self) -> &mut dyn Car;
}
