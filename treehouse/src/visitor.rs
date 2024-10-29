/*
Rust has two types for strings
str
String
str is a string slice, a reference to a UTF-8 encoded string in memory.
String is a heap-allocated string. It is growable, mutable and owned.
*/
use crate::VisitorAction;

#[derive(Debug)]
pub struct Visitor {
    pub name: String,
    pub action: VisitorAction,
    age: i8
}

impl Visitor {
    pub fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age
        }
    }

    pub fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the treehouse, {}", self.name),
            VisitorAction::AcceptWithNote {note } => {
                println!("Welcome to the treehouse, {}", self.name);
                println!("{note}");
                if self.age < 18 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is now a probationary member", self.name),
            VisitorAction::Refuse => println!("Sorry, we have to refuse you entry"),
            _ => println!("Sorry, we have to refuse you entry")
        }
    }
}