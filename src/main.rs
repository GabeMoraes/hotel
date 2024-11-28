use std::io;

fn show_menu() {
    println!("Welcome to the check in/out controller.");
    println!("Please input an option:");
    println!("1 - Check in guest");
    println!("2 - Exit");
}

fn main() {
    loop {
        show_menu();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        println!("You have chosen: {}", input);
        
        if input == "2\n" {
            break;
        }
    }
}