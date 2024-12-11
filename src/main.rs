use std::io;
use serde::{Deserialize, Serialize};

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

#[derive(PartialEq, Eq)]
enum Gender {
    Masc,
    Fem,
    Nb
}

#[derive(PartialEq, Eq)]
enum PayMethod {
    Credit,
    Cash
}

fn check_in() {
    let mut name = String::new();
    let mut id: u64;
    let mut birth_dt = String::new();
    let mut gender:Gender;
    let mut address_st = String::new();
    let mut address_n = String::new();
    let mut postal_code: u32;
    let mut tel_number = String::new();
    let mut pay_method: PayMethod;
    
    println!("Please input guest name:");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line.");

    let name = name.trim();

    println!("{}", name);

    println!("Please input guest id:");

    let mut input_line = String::new();
    
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line.");

    println!("{}", input_line);

    id = input_line.trim().parse().expect("Input not int");

    println!("{}", id);

    println!("Please input guest birth:");
    
    io::stdin()
        .read_line(&mut birth_dt)
        .expect("Failed to read line.");

    let birth_dt = birth_dt.trim();

    println!("{}", birth_dt);

    println!("Please input guest gender:");

    let mut input_line = String::new();

    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line.");

    if input_line.trim() == "Masc" {
        gender = Gender::Masc;
    }
    else {
        gender = Gender::Fem;
    }

    if gender == Gender::Masc {
        println!("Masc");
    }
    else if gender == Gender::Fem {
        println!("Fem");
    }

    println!("Please input guest street:");

    io::stdin()
        .read_line(&mut address_st)
        .expect("Failed to read line.");

    let address_st = address_st.trim();

    println!("{}", address_st);

    println!("Please input guest address number:");

    io::stdin()
        .read_line(&mut address_n)
        .expect("Failed to read line.");

    let address_n = address_n.trim();

    println!("{}", address_n);

    println!("Please input guest postal code:");

    let mut input_line = String::new();
    
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line.");

    println!("{}", input_line);

    postal_code = input_line.trim().parse().expect("Input not int");

    println!("{}", postal_code);

    println!("Please input guest telephone number:");

    io::stdin()
        .read_line(&mut tel_number)
        .expect("Failed to read line.");

    let tel_number = tel_number.trim();

    println!("{}", tel_number);

    println!("Please input guest's payment method:");

    let mut input_line = String::new();

    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line.");

    if input_line.trim() == "Credit" {
        pay_method = PayMethod::Credit;
    }
    else {
        pay_method = PayMethod::Cash;
    }

    if pay_method == PayMethod::Credit {
        println!("Credit");
    }
    else if pay_method == PayMethod::Cash {
        println!("Cash");
    }
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
            check_in();
        }
    }
}