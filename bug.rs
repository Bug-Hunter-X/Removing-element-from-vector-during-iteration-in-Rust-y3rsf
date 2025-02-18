fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let mut index = 0;

    loop {
        if index >= numbers.len() {
            break;
        }

        let value = numbers[index];

        if value == 3 {
            numbers.remove(index);
        } else {
            index += 1;
        }
    }

    println!("Numbers after removing 3: {:?}", numbers);
}