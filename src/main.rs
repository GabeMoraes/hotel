use std::io;
use std::fs::{self, File};
use std::io::{Write, Read};

use serde::{Serialize, Deserialize};
use bincode;
use chrono::{self, DateTime, Local};

#[derive(Serialize, Deserialize, Debug)]
struct Guest {
    name: String,
    id: u64,
    birth: String,
    gender: Gender,
    address_st: String,
    address_n: String,
    postal_code: u32,
    tel_number: String,
    pay_method: PayMethod,
    checkin_time: DateTime<Local>
}

//impl Guest {
//    const FIELD_COUNT: u8 = 9;
//}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
enum Gender {
    Masc,
    Fem,
    Nb
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
enum PayMethod {
    Credit,
    Cash
}

fn input_msg(field: String) {
    println!("Please input guest's {}", field);
}

fn check_in() {
    let mut name = String::new();
    let id: u64;
    let mut birth = String::new();
    let gender:Gender;
    let mut address_st = String::new();
    let mut address_n = String::new();
    let postal_code: u32;
    let mut tel_number = String::new();
    let pay_method: PayMethod;
    
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
        .read_line(&mut birth)
        .expect("Failed to read line.");

    let birth = birth.trim();

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
        birth.to_string(),
        gender,
        address_st.to_string(),
        address_n.to_string(),
        postal_code,
        tel_number.to_string(),
        pay_method,
        Local::now()
    );

    println!("{:?}", new_guest);

    let bin_data = bincode::serialize(&new_guest).unwrap();

    let file = File::create(new_guest.id.to_string() + ".bin");
    let _ = file.expect("REASON").write_all(&bin_data);
    println!("Data saved to '{}.bin'.", new_guest.id);

    // Lendo o arquivo binário.
    let mut bin_data_from_file = Vec::new();
    let file = File::open(new_guest.id.to_string() + ".bin");
    let _ = file.expect("REASON").read_to_end(&mut bin_data_from_file);

    // Desserialização: Recuperando a struct a partir dos dados binários.
    let new_guest: Guest = bincode::deserialize(&bin_data_from_file).unwrap();
    println!("Deserialized data: {:#?}", new_guest);
}

fn check_out() {
    input_msg("id".to_string());
    
    let mut id = String::new();
    
    io::stdin()
        .read_line(&mut id)
        .expect("Failed to read line.");

    id = id.trim().to_string();

    let path = format!("{}{}", id, ".bin");

    println!("{}", path);

    // Removendo o arquivo
    match fs::remove_file(path) {
        Ok(_) => println!("Guest w/ id {} checked out successully.", id),
        Err(e) => eprintln!("Error trying to check out guest: {}", e),
    }
}

fn display_guest() {
    input_msg("id".to_string());

    let mut id = String::new();
    
    io::stdin()
        .read_line(&mut id)
        .expect("Failed to read line.");

    id = id.trim().to_string();

    let path = format!("{}{}", id, ".bin");

    let mut bin_data_from_file = Vec::new();
    let file = File::open(path);
    let _ = file.expect("REASON").read_to_end(&mut bin_data_from_file);

    let guest: Guest = bincode::deserialize(&bin_data_from_file).unwrap();
    
    println!("Name: {}", guest.name);
    println!("ID: {}", guest.id);
    println!("Birth: {}", guest.birth);
    println!("Gender: {:?}", guest.gender);
    println!("Street: {}", guest.address_st);
    println!("Number: {}", guest.address_n);
    println!("Postal code: {}", guest.postal_code);
    println!("Telephone number: {}", guest.tel_number);
    println!("Payment method: {:?}", guest.pay_method);
}

fn update_guest() {
    let mut delete_flag: bool = false;
    let mut old_id: u64 = 0;

    input_msg("id".to_string());

    let mut id = String::new();
    
    io::stdin()
        .read_line(&mut id)
        .expect("Failed to read line.");

    id = id.trim().to_string();

    let path = format!("{}{}", id, ".bin");

    let mut bin_data_from_file = Vec::new();
    let file = File::open(path);
    let _ = file.expect("REASON").read_to_end(&mut bin_data_from_file);

    let mut guest: Guest = bincode::deserialize(&bin_data_from_file).unwrap();
    
    println!("Select field to change.");
    println!("Please input an option:");
    println!("1 - Name");
    println!("2 - ID");
    println!("3 - Birth");
    println!("4 - Gender");
    println!("5 - Address street");
    println!("6 - Address number");
    println!("7 - Postal code");
    println!("8 - Telephone number");
    println!("9 - Payment method");
    
    let mut input_line = String::new();

    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");

    if input_line.trim() == "1" {
        let mut input_line = String::new();
        
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");

        guest.name = input_line.trim().to_string();
    }

    else if input_line.trim() == "2" {
        old_id = guest.id;

        delete_flag = true;

        let mut input_line = String::new();
        
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");

        guest.id = input_line.trim().parse().expect("Not int");
    }

    else if input_line.trim() == "3" {
        let mut input_line = String::new();
        
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");

        guest.birth = input_line.trim().to_string();
    }

    else if input_line.trim() == "4" {
        let mut input_line = String::new();
        
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");

        if input_line.trim() == "Masc" {
            guest.gender = Gender::Masc;
        }

        else if input_line == "Fem" {
            let _ = guest.gender == Gender::Fem;
        }

        else {
            let _ = guest.gender == Gender::Nb;
        }
    }

    else if input_line.trim() == "5" {
        let mut input_line = String::new();
        
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");

        guest.address_st = input_line.trim().to_string();
    }

    else if input_line.trim() == "6" {
        let mut input_line = String::new();
        
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");

        guest.address_n = input_line.trim().to_string();
    }

    else if input_line.trim() == "7" {
        let mut input_line = String::new();
        
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");

        guest.tel_number = input_line.trim().parse().expect("Not int");
    }

    else if input_line.trim() == "8" {
        let mut input_line = String::new();
        
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");

        guest.tel_number = input_line.trim().to_string();
    }

    else if input_line.trim() == "9" {
        let mut input_line = String::new();
        
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");

        if input_line.trim() == "Credit" {
            guest.pay_method = PayMethod::Credit;
        }

        else if input_line == "Cash" {
            let _ = guest.pay_method == PayMethod::Cash;
        }
    }

    else {
        println!("Illegal option.");
        return;
    }

    println!("{:?}", guest);

    let bin_data = bincode::serialize(&guest).unwrap();

    let file = File::create(guest.id.to_string() + ".bin");
    let _ = file.expect("REASON").write_all(&bin_data);
    println!("Data saved to '{}.bin'.", guest.id);

    if delete_flag {
        // Removendo o arquivo
        match fs::remove_file(old_id.to_string() + ".bin") {
            Ok(_) => println!("Id {} changed", id),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    // Lendo o arquivo binário.
    let mut bin_data_from_file = Vec::new();
    let file = File::open(guest.id.to_string() + ".bin");
    let _ = file.expect("REASON").read_to_end(&mut bin_data_from_file);

    // Desserialização: Recuperando a struct a partir dos dados binários.
    let guest: Guest = bincode::deserialize(&bin_data_from_file).unwrap();
    println!("Deserialized data: {:#?}", guest);
}

fn build_guest(
        name: String, id: u64, birth: String, gender: Gender,
        address_st: String, address_n: String, postal_code: u32,
        tel_number: String, pay_method: PayMethod, checkin_time: DateTime<Local>) -> Guest {
    Guest {
        name,
        id,
        birth,
        gender,
        address_st,
        address_n,
        postal_code,
        tel_number,
        pay_method,
        checkin_time
    }
}

fn show_menu() {
    println!("Welcome to the check in/out controller.");
    println!("Please input an option:");
    println!("1 - Check in guest");
    println!("2 - Check out guest");
    println!("3 - Display guest");
    println!("4 - Update guest");
    println!("\n0 - Exit");
}

fn main() {
    loop {
        show_menu();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        println!("You have chosen: {}", input);
        
        if input == "0\n" {
            break;
        }

        else if input == "1\n" {
            check_in();
        }

        else if input == "2\n" {
            check_out();
        }

        else if input == "3\n" {
            display_guest();
        }

        else if input == "4\n" {
            update_guest();
        }
    }
}