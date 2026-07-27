mod bit_to_dec;
mod dec_to_bit;
use std::io;

use crate::{bit_to_dec::mode_1, dec_to_bit::mode_2};

fn main() {
    println!("Welcome to the Yaeyany Bit calculating trainer.");
    println!("Please select a mode:");
    println!("1. Convert given binary to decimal");
    println!("2. Convert given decimal to binary");
    println!("0. Quit");    
    
    loop {

        let mut mode_choice = String::new();

        io::stdin().read_line(&mut mode_choice).expect("Failed_mode_choice");
        let mode_choice_trim = mode_choice.trim().to_lowercase();
        
        match mode_choice_trim.as_str() {
            "0"|"q"| "quit"| "exit"| "cancel" => {
                println!("Goodbye");
                break;
            },
            "1" => {
                mode_1();
            }
            "2" => {
                mode_2();
            }
            "h" => {
                println!("1. Convert given binary to decimal");
                println!("2. Convert given decimal to binary");
                println!("0. Quit");   
            }
            _ => println!("Invalid input. Please enter a valid mode. 'H' to help"),
        };
    }
}


