#![warn(clippy::all, clippy::pedantic)]

mod arrays;
mod visitor;

use std::io::stdin;
use visitor::Visitor;

fn get_name_input() -> String {
    let mut name = String::new();

    // by using & we are passing a reference to the variable name
    // by using mut we are allowing the variable to be mutable
    stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    name
}

fn main() {
    println!("Hello, what is your name?");

    let name = get_name_input();
    
    println!("Hello, {name}");
    println!("{:?} is {} characters long", name, name.len());
    println!("{:?} trimmed name", name.trim());

    if name.trim().to_lowercase() == "Kuba".to_lowercase() {
        println!("You are Kuba");
    }
    else {
        println!("You are not Kuba");
    }

    arrays::run();

    println!("Can Kuba enter? {}", arrays::allow("Kuba"));
    let mut result1 = arrays::allow("Kuba");
    println!("Can Kuba enter? {result1}");
    println!("Can John enter? {}", arrays::allow("John"));
    result1 = arrays::allow("John");
    println!("Can John enter? {result1}");

    visitors();
}

fn visitors() {
    
    let visitors_list = [
        Visitor::new("Kuba", "Hello Kuba"),
        Visitor::new("John", "Hello John"),
        Visitor::new("Jane", "Hello Jane"),
    ];

    let allowed_visitors = visitors_list
        .iter()
        .find(|visitor| visitor.name == "Kuba");

    match allowed_visitors {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("No visitors allowed"),
    }
}
