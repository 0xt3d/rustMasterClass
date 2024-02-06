fn main() {
    let s1 = String::from("hello");
    
    // `&s1` is a reference to the string owned by s1
    let len = calculate_length(&s1);
    
    println!("The length of '{}' is {}.", s1, len);
}

// The function takes a reference to a string
fn calculate_length(s: &String) -> usize {
    s.len()
}
