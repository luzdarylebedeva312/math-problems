use std::vec;
fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let target = 7;
    let mut sum = 0;
    for num in &numbers {
        if *num == target {
            return true;
        }
        sum += num;
    }
    return false;
}
