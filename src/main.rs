// import user input package
use std::io;

fn main() {
    let mut input = String::new();
    
    println!("Feed me your first input");
    
    // create input
    io::stdin()
    // & symbol to create reference
    // this gives us a modifiable reference to the input variable
    // returns result object
    .read_line(&mut input)
    // .expect checks if .read_line gives back a valid value
    .expect("Failed to read line");
    
    println!("{}", input);
}
