mod calculator;
mod calculation_interpreter;
//<>

use std::io;
use crate::calculation_interpreter::calculation_interpreter::calculate_string;

fn main() {
    while true {
        println!("Please input your calculation!");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let user_input = user_input.trim();

        if user_input == "exit" {
            println!("Exiting program!");
            break;
        }

        calculate_string(user_input.trim());
    }
}


