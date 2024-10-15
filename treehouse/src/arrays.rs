pub fn run()
{
    println!("Arrays");

    let array = [1, 2, 3, 4, 5];
    let string_array = ["Hello", "World", "Rust"];

    for element in &array {
        println!("Value: {element}");
    }

    /*
    Rust
can directly access the contents of an array or other container, without
needing to use index numbers. This is shorter and safer. You can’t mess up
your index numbers and crash your program by accessing an element that
doesn’t exist.
     */
    for element in &string_array {
        println!("Value: {element}");
    }

    for i in 0..10 {
        println!("Value: {i}");
    }
}

pub fn allow(name: &str) -> bool
{
    let list_of_names = ["Kuba", "Norbert", "Doe"];

    let mut allow_them_in = false;

    for value in &list_of_names
    {
        if value.to_lowercase() == name.to_lowercase() {
            allow_them_in = true;
        }
    }

    allow_them_in
}