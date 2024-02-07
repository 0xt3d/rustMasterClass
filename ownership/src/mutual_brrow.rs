fn main() {
    let mut s = String::from("hello");
    
    // Mutable reference to the string owned by s
    change_string(&mut s);

    println!("The modified string is: {}", s);
}

// The function takes a mutable reference to a string
fn change_string(s: &mut String) {
    s.push_str(", world!");
}
