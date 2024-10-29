#![warn(clippy::all, clippy::pedantic)]

mod arrays;
mod visitor;

use std::io::stdin;
use visitor::Visitor;
use crate::VisitorAction::{Accept, AcceptWithNote, Refuse, Probation};

fn main() {
    // println!("Hello, what is your name?");
    //
    // let name = get_name_input();
    //
    // println!("Hello, {name}");
    // println!("{:?} is {} characters long", name, name.len());
    // println!("{:?} trimmed name", name.trim());
    //
    // if name.trim().to_lowercase() == "Kuba".to_lowercase() {
    //     println!("You are Kuba");
    // }
    // else {
    //     println!("You are not Kuba");
    // }
    //
    // arrays::run();
    //
    // println!("Can Kuba enter? {}", arrays::allow("Kuba"));
    // let mut result1 = arrays::allow("Kuba");
    // println!("Can Kuba enter? {result1}");
    // println!("Can John enter? {}", arrays::allow("John"));
    // result1 = arrays::allow("John");
    // println!("Can John enter? {result1}");

    visitors();
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

fn get_name_input() -> String {
    let mut name = String::new();

    // by using & we are passing a reference to the variable name
    // by using mut we are allowing the variable to be mutable
    stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    name
}

fn what_is_your_name() -> String {
    println!("What is your name?");

    get_name_input()
}

fn visitors() {

    let mut visitor_list = vec![
        Visitor::new("Kuba", Accept, 36),
        Visitor::new("Jane", AcceptWithNote { note: String::from("Loves dogs") }, 24),
        Visitor::new("John", Refuse, 12),
    ];

    loop {
        println!("Hello, what's your name? (Leave empty and press ENTER to quit)");
        let name = what_is_your_name();
        println!("{name:?}");

        let name = name.trim();
        println!("{name:?}");

        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name.to_lowercase());

        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty()
                {
                    break;
                }
                else
                {
                    println!("{name} is not on the visitor list.");
                    visitor_list.push(Visitor::new(&name, Probation, 0));
                }
            }
        }
    }

    println!("The final list of visitors:");
    println!("{:#?}", visitor_list);
}
