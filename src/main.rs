use std::io;

fn main() {
    println!("Enter your weight in kilograms: ");

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let weight : f32 = input.trim().parse().unwrap();

    println!("The weight in Mars is {}", return_mars_weight(weight)); 
}

fn return_mars_weight (weight : f32) -> f32 {
    (weight/9.81)*3.7111
}

// RUST OWNERSHIP PRINCIPLE
// 1. Each value is owned by a variable
// 2. variable goes out of scope so does the values.
// 3. A value can be owned by one variable only during the scope.

// RUST BORROWING PRINCIPLE
// 1. When a non-primitive value is owned by one variable, other functions and variables call its reference(memory location).
// 2. There can only be one mutable reference to a value during a scope