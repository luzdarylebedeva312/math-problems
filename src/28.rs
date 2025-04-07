use std::vec::Vec;

fn main() {
    let vec1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut vec2: Vec<i32> = vec1.clone();

    vec2.push(6);
    
    println!("{:?}", vec1);
    println!("{:?}", vec2);
}
