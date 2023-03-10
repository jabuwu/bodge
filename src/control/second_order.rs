use std::{
    f32::consts::PI,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Default, Clone, Copy)]
pub struct SecondOrder<T> {
    pub input_previous: T,
    pub output: T,
    pub output_velocity: T,

    pub k1: f32,
    pub k2: f32,
    pub k3: f32,
}

impl<
        T: Default
            + Copy
            + Sub<T, Output = T>
            + Div<f32, Output = T>
            + Add<T, Output = T>
            + Mul<f32, Output = T>,
    > SecondOrder<T>
{
    pub fn new(initial: T, k1: f32, k2: f32, k3: f32) -> SecondOrder<T> {
        SecondOrder {
            input_previous: initial,
            output: initial,
            output_velocity: T::default(),
            k1,
            k2,
            k3,
        }
    }

    pub fn new_frequency_response(
        initial: T,
        frequency: f32,
        response: f32,
        damping: f32,
    ) -> SecondOrder<T> {
        let mut second_order = SecondOrder::new(initial, 0., 0., 0.);
        second_order.set_frequency_response(frequency, response, damping);
        second_order
    }

    pub fn update(&mut self, input: T, delta_seconds: f32) -> T {
        if delta_seconds == 0. {
            return self.output;
        }
        let k2_stable = self
            .k2
            .max(delta_seconds * delta_seconds / 2. + delta_seconds * self.k1 / 2.)
            .max(delta_seconds * self.k1);
        let vec_velocity = (input - self.input_previous) / delta_seconds;
        self.input_previous = input;
        self.output = self.output + self.output_velocity * delta_seconds;
        self.output_velocity = self.output_velocity
            + (input + vec_velocity * self.k3 - self.output - self.output_velocity * self.k1)
                / k2_stable
                * delta_seconds;
        self.output
    }

    pub fn set_frequency_response(&mut self, frequency: f32, response: f32, damping: f32) {
        self.k1 = damping / (PI * frequency);
        self.k2 = 1. / (2. * PI * frequency).powf(2.);
        self.k3 = response * damping / (2. * PI * frequency);
    }
}
