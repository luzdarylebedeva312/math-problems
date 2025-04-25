fn main() {
    // Example problem: Sum of squares
    let numbers = vec![1, 2, 3, 4];
    println!("The sum of squares is: {}", (numbers.iter().map(|&x| x.pow(2)).sum()));
}
