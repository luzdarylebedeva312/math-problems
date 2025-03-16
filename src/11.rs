fn main() {
    let mut rng = rand::thread_rng();

    // Generate a random integer between 1 and 10
    let x: i32 = rng.gen_range(1..=10);

    // Calculate the square root of x
    let y: f64 = x.sqrt();

    println!("The square root of {} is {}.", x, y);
}
