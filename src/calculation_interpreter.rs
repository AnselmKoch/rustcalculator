pub mod calculation_interpreter {
    use std::collections::HashMap;
    use std::time::Instant;
    use crate::calculator::calculator::{calculate, CalculationElement};
    //<>
    //(1+(1+4))


    pub fn calculate_string(calculation: &str) {
        let start_time = Instant::now();
        let calc = remove_whitespaces(calculation);

        let mut no_bracket_calc = process_brackets(calc);
        no_bracket_calc = remove_whitespaces(&no_bracket_calc);

        let value = calculate(&no_bracket_calc);

        let elapsed_time = start_time.elapsed().as_micros();


        println!("Finished calculating... Solution is: \n{} \nit took {} MICRO-SECONDS to calculate", value.to_string(), elapsed_time);
    }

    fn process_brackets (mut string: String) -> String {
        println!("processing brackets for string: \n{}", string);
        let mut opening_bracket_index: u128 = 0;
        let mut closing_bracket_index: u128 = 0;

        for (index, char) in string.char_indices() {
            if is_bracket(char) && !is_closing_bracket(char) {
                opening_bracket_index = index as u128;
                continue;
            }

            if is_closing_bracket(char) {
                closing_bracket_index = index as u128;
                break;
            }
        }
        if closing_bracket_index == 0 {
            return string
        }

        let content: String = string[(opening_bracket_index + 1) as usize .. (closing_bracket_index) as usize].to_string();

        println!("bracket information: open: {}   closed: {}", opening_bracket_index, closing_bracket_index);
        println!("Content of bracket: {}", content);
        let bracket_value = calculate(&content);

        println!("Done calculating value: {}", bracket_value);

        for i in 0..closing_bracket_index + 1- opening_bracket_index {
            string.remove(opening_bracket_index as usize);
        }

        println!("Removed bracket from string!\n{}\n", string);

        string.insert_str(opening_bracket_index as usize, &bracket_value.to_string());

        println!("Injected value into string! \n{}\n", string);

        return process_brackets(string)
    }

    fn remove_whitespaces(string: &str) -> String {
        string.chars().filter(|c| !c.is_whitespace()).collect()
    }

    fn is_bracket(char: char) -> bool {
        char == '(' || char == ')'
    }

    fn is_closing_bracket(char: char) -> bool {
        char == ')'
    }





    //This was the old - very bad function. now replaced with "process_brackets"

    /*
    fn process_brackets1(string: &str) -> Vec<TotalCalculation> {
        let collection = Vec::new();
        //(1+(1+4))

        let mut bracket_maps: HashMap<(u128), Vec<Brackets>> = HashMap::new();
        let mut bracket_depth: u128 = 0;
        let mut open_bracket_cache:  Vec<SingleBracket> = Vec::new();
        let mut content_cache: Vec<Vec<char>> = Vec::new();
        for (index, char) in string.char_indices() {
            println!("Current char!{}", char);
            let mut  current_char_array = Vec::new();

            if content_cache.len() != 0 {
                current_char_array = content_cache.pop().expect("Error popping! {}");
            }
            //if char is not a bracket skip this iteration
            if !is_bracket(char) {
                if bracket_depth > 0 {
                    current_char_array.push(char);
                    content_cache.push(current_char_array);
                }
                continue;
            }

            let is_closing_bracket = is_closing_bracket(char);

            //if current bracket is a closing one and no opening bracket was cached the syntax is incorrect
            // because there cant be a closing bracket before a closing bracket.
            if is_closing_bracket && open_bracket_cache.len() == 0usize {
                return collection
            }

            let current_bracket = SingleBracket::new();

            if !is_closing_bracket {
                open_bracket_cache.push(current_bracket);
                bracket_depth += 1;


                let new_array = Vec::new();
                content_cache.push(current_char_array);
                content_cache.push(new_array);
            } else {
                //closing bracket! Bracket is done!

                let temp_string: String = current_char_array.into_iter().collect();
                println!("Bracket is done!! {}", temp_string);
                println!("Current depth: {}", &bracket_depth);

                let bracket = Brackets::new(open_bracket_cache.pop().unwrap(), current_bracket, temp_string.clone());
                let current_depth_brackets = bracket_maps.get_mut(&bracket_depth);

                let bracket_value = bracket.calculate_bracket();

                println!("BRACKET VALUE! {}", bracket_value);

                match current_depth_brackets {
                    Some(value) => {
                        value.push(bracket);
                    }
                    None  => {
                        bracket_maps.insert(bracket_depth, vec![bracket]);
                    }
                }

                println!("Bracketmap!!{}", bracket_maps.len());

                bracket_depth -= 1;
                if bracket_depth == 0 {
                    continue;
                }
                let new_char_array = content_cache.pop().unwrap();
                content_cache.push(new_char_array);
            }
        }
        collection
    }

     */

}