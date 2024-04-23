use std::io;
use rand::prelude::*;

fn main() {

    
    let mut running: bool = true;
    let mut password: String = String::new();
    let mut password_length: i32 = 0;
    let mut special_chars_access: bool = false;
    let mut capital_letters_access: bool = false;
    let mut numbers_access: bool = false;

    println!("// Password-generator //");
    
    while running {
        let mut input = String::new();
        println!("Choose password length (max 255):");
        io::stdin().read_line(&mut input).expect("Failed to read line");

        
        while password_length_check(&input) == false {
            input.clear();
            println!("Length must be given in numbers only!");
            io::stdin().read_line(&mut input).expect("Failed to read line");
            if password_length_check(&input) {
                break
            }
        }
        password_length = convert_to_int(&input);
        println!("Password length is: {}", password_length);
        
        input.clear();
        println!("Do you want capital letters? (Y/n)");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim().to_lowercase() == "y" {
            capital_letters_access = true;
        }

        input.clear();
        println!("Do you want numbers in password? (Y/n");
        io::stdin().read_line(&mut input).expect("Could not read inout");
        if input.trim().to_lowercase() == "y" {
            numbers_access = true;
        }

        input.clear();
        println!("Do you want special characters? (Y/n)");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim().to_lowercase() == "y" {
            special_chars_access = true;
        }

        password = generate_password(capital_letters_access, numbers_access, special_chars_access, password_length);
        println!("Password is: {password}");
        
        running = false;
    }
    

}

fn password_length_check(length_in: &str) -> bool {
    let length_in_int: Result<i32, _> = length_in.trim().parse();
    match length_in_int {
        Ok(_) => return true,
        Err(_) => return false,
    }
}

fn convert_to_int(input_string: &str) -> i32 {
    let output_int: i32 = input_string.trim().parse().unwrap();
    return output_int;
}

fn generate_password(capital_chars: bool, numbers: bool, special_chars: bool, length: i32) -> String {
    let chars: [&str;26] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
    
    let mut chars_upper: Vec<String> = vec![];
    for &char in &chars {
        chars_upper.push(char.to_uppercase());
    }

    let numbers_chars: [&str;10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let specials: [&str;7] = ["!", "?", "#", "^", "%", "&", "="];

    // pool of all chars used to generate password
    let mut password_pool: Vec<String> = vec![];
    
    // final password
    let mut password = String::from("");

    // lower chars
    for char in chars {
        password_pool.push(char.to_string());
    }

    // capital letters
    if capital_chars == true {
        for char in chars_upper {
            password_pool.push(char);
        }
    }

    // numbers
    if numbers == true {
        for num in numbers_chars {
            password_pool.push(num.to_string());
        }
    }

    // special chars
    if special_chars == true {
        for special in specials {
            password_pool.push(special.to_string());
        }
    }

    for _i in 0..=length {
        let random: usize = random_number(password_pool.len());
        let new_char = &password_pool[random];
        password += new_char;
    }


    return password;
    
}

fn random_number(range: usize) -> usize {

    let mut rng = rand::thread_rng();
    let num: usize = rng.gen_range(0..range);
    return num;

}