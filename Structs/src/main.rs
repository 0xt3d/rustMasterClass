struct Person {
    name: String,
    age: u32,
    is_student: bool,
}

fn main() {
    // Creating an instance of the Person struct
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
        is_student: false,
    };

    // Accessing struct fields
    println!("Name: {}", person1.name);
    println!("Age: {}", person1.age);
    println!("Is Student: {}", person1.is_student);
}
