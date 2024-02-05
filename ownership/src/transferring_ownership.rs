fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // Ownership of the string is moved from s1 to s2

    // Uncommenting the next line would result in a compilation error
    // println!("{}", s1); // Error: value moved
    println!("{}", s2); // OK: s2 owns the string now
}
