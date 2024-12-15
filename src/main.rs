use std::io;
use serde::{Deserialize, Serialize};
use bincode;
use std::fs::{self, File};
use std::io::{Write, Read};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Guest {
    name: String,
    id: u64,
    birth_dt: String,
    gender: Gender,
    address_st: String,
    address_n: String,
    postal_code: u32,
    tel_number: String,
    pay_method: PayMethod
}

#[derive(PartialEq, Eq, Debug, serde::Deserialize, serde:: Serialize)]
enum Gender {
    Masc,
    Fem,
    Nb
}

#[derive(PartialEq, Eq, Debug, serde::Deserialize, serde:: Serialize)]
enum PayMethod {
    Credit,
    Cash
}

fn input_msg(field: String) {
    println!("Please input guest's {}", field);
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
    
    input_msg("name".to_string());

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line.");

    let name = name.trim();

    input_msg("id".to_string());

    let mut input_line = String::new();
    
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line.");

    id = input_line.trim().parse().expect("Input not int");

    input_msg("birth date".to_string());
    
    io::stdin()
        .read_line(&mut birth_dt)
        .expect("Failed to read line.");

    let birth_dt = birth_dt.trim();

    input_msg("gender".to_string());

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

    input_msg("street name".to_string());

    io::stdin()
        .read_line(&mut address_st)
        .expect("Failed to read line.");

    let address_st = address_st.trim();

    input_msg("address number".to_string());

    io::stdin()
        .read_line(&mut address_n)
        .expect("Failed to read line.");

    let address_n = address_n.trim();

    input_msg("postal code".to_string());

    let mut input_line = String::new();
    
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line.");

    postal_code = input_line.trim().parse().expect("Input not int");

    input_msg("telephone number".to_string());

    io::stdin()
        .read_line(&mut tel_number)
        .expect("Failed to read line.");

    let tel_number = tel_number.trim();

    input_msg("payment method".to_string());

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

    let new_guest = build_guest(
        name.to_string(),
        id,
        birth_dt.to_string(),
        gender,
        address_st.to_string(),
        address_n.to_string(),
        postal_code,
        tel_number.to_string(),
        pay_method);

    println!("{:?}", new_guest);

    let bin_data = bincode::serialize(&new_guest).unwrap();

    let mut file = File::create(new_guest.id.to_string() + ".bin");
    file.expect("REASON").write_all(&bin_data);
    println!("Data saved to '{}.bin'.", new_guest.id);

    // Lendo o arquivo binário.
    let mut bin_data_from_file = Vec::new();
    let mut file = File::open(new_guest.id.to_string() + ".bin");
    file.expect("REASON").read_to_end(&mut bin_data_from_file);

    // Desserialização: Recuperando a struct a partir dos dados binários.
    let new_guest: Guest = bincode::deserialize(&bin_data_from_file).unwrap();
    println!("Deserialized data: {:#?}", new_guest);
}

fn build_guest(
    name: String,
    id: u64,
    birth_dt: String,
    gender: Gender,
    address_st: String,
    address_n: String,
    postal_code: u32,
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