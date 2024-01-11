use std::io;

fn main() {
    println!("Please enter the base directory to check the code length.");
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("failed to read line ...");
    println!("You entered: {}", input_string);
}
