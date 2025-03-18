use std::f64::consts::PI;
fn calculate_circumference(radius: f64) -> f64 {
    2 * PI * radius
}
fn main() {
    let radius = 5.0;
    println!("The circumference of a circle with a radius of {} is {}", radius, calculate_circumference(radius));
}