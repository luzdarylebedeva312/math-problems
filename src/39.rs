// Problem: Write a Rust function that takes an integer and returns its factorial.
fn main() {
    let num = 5; // Replace with any integer you want to calculate the factorial of
    let result = factorial(num);
    println!("The factorial of {} is {}", num, result);
}

/// Calculates the factorial of a given number using recursion.
fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}
