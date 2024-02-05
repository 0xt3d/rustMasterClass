fn main() {
    // Integer variable `x` owns the value 5
    let x = 5;

    // String variable `s` owns the string "hello"
    let s = String::from("hello");

    // When `x` goes out of scope, nothing special happens (it's a simple copy)
    // But when `s` goes out of scope, the memory for the string is freed
}