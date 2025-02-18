fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];

    numbers.retain(|&x| x != 3);

    println!("Numbers after removing 3: {:?}", numbers);
}

//Alternative solution using iterators
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let numbers_without_3: Vec<i32> = numbers.into_iter().filter(|&x| x != 3).collect();
    println!("Numbers after removing 3: {:?}", numbers_without_3);
} 