use std::io;

struct Guest {
    name: String,
    id: u32,
    birth_dt: String,
    gender: Gender,
    address_st: String,
    address_n: String,
    postal_code: u16,
    tel_number: String,
    pay_method: PayMethod
}

enum Gender {
    Masc,
    Fem,
    Nb
}

enum PayMethod {
    Credit,
    Cash
}

fn check_in() {
    let mut name = String::new();
    let mut id: u32;
    let mut birth_dt = String::new();
    let mut gender:Gender;
    let mut address_st = String::new();
    let mut address_n = String::new();
    let mut postal_code: u16;
    let mut tel_number = String::new();
    let mut pay_method: PayMethod;
    
    println!("Please input guest name:");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let name = name.trim();
}

fn build_guest(
    name: String,
    id: u32,
    birth_dt: String,
    gender: Gender,
    address_st: String,
    address_n: String,
    postal_code: u16,
    tel_number: String,
    pay_method: PayMethod
    ) -> Guest {
        Guest {
            name: name,
            id: id,
            birth_dt: birth_dt,
            gender: gender,
            address_st: address_st,
            address_n: address_n,
            postal_code: postal_code,
            tel_number: tel_number,
            pay_method: pay_method
        }
}

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

        if input == "1\n" {

        }
    }
}