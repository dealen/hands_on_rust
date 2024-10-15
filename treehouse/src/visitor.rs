/*
Rust has two types for strings
str
String
str is a string slice, a reference to a UTF-8 encoded string in memory.
String is a heap-allocated string. It is growable, mutable and owned.
*/

pub struct Visitor
{
    pub name: String,
    pub greeting: String,
}

impl Visitor
{
    #[must_use]
    pub fn new(name: &str, greeting: &str) -> Self
    {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    pub fn greet_visitor(&self)
    {
        println!("{}", self.greeting);
    }
}