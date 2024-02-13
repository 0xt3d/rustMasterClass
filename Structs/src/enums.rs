enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let some_number = Some(5);
    let absent_number: Option<i32> = None;

    match some_number {
        Some(value) => println!("The value is: {}", value),
        None => println!("No value!"),
    }

    match absent_number {
        Some(value) => println!("The value is: {}", value),
        None => println!("No value!"),
    }
}
