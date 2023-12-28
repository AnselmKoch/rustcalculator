
pub mod calculator {

    pub struct CalculationPart<'a> {
        pub a: &'a CalculationElement,
        pub b: &'a CalculationElement,
        pub operator: &'a CalculationElement,
    }

    impl<'a> CalculationPart<'a> {
        pub fn new(a: &'a CalculationElement, b: &'a CalculationElement, operator: &'a CalculationElement) -> CalculationPart<'a> {
            CalculationPart{
                a, b, operator,
            }
        }

        pub fn to_string(&self) -> String {
            format!("CALCULATION PART: \noperator: {} \na: {}\nb: {}", self.operator.char, self.a.char, self.b.char)
        }
    }


    pub fn string_to_calculation_parts(content: &String) -> Vec<CalculationElement> {
        let content = content;

        let mut calculation_elements: Vec<CalculationElement> = Vec::new();

        //String to put together individual chars to one big number (2353 or 5.123821) for example
        let mut current_number_cache: String = String::new();

        //12+4*5+6
        for (index, char) in content.char_indices() {
            let is_operation_char = CalculationOperation::operation_from_char(&String::from(char)) != CalculationOperation::Null;

            if index == content.len() - 1 {
                if (is_operation_char) {
                    calculation_elements.push(CalculationElement::new(ElementType::Number, current_number_cache.clone()));
                    calculation_elements.push(CalculationElement::new(ElementType::Operation, String::from(char)));
                    continue;
                }
                current_number_cache.push(char);
                calculation_elements.push(CalculationElement::new(ElementType::Number, current_number_cache.clone()));
            }

            //is a +,-,*,/ or whatever...
            if is_operation_char {
                //last number string is finished
                let last_number_as_string: String = current_number_cache.clone();
                current_number_cache = String::new();

                let temp_calc_element = CalculationElement::new(ElementType::Number, last_number_as_string.clone());
                calculation_elements.push(temp_calc_element);
                calculation_elements.push(CalculationElement::new(ElementType::Operation, String::from(char)));
                if !is_number(&last_number_as_string) {
                    return Vec::new();
                }

            } else {
                current_number_cache.push(char);
            }

        }
        calculation_elements
    }

    pub fn calculate_calc_elements(elements: Vec<CalculationElement>, brackets: &Brackets) -> f64{
        let mut elements = elements.clone();

        elements = calculate_all_priority_operators(elements, brackets);

        elements = calculate_all_default_operators(elements, brackets);

        if elements.len() == 1 {
            return elements.get(0).unwrap().char.parse::<f64>().unwrap()
        } else {
            return 0.0
        }
    }

    fn calculate_all_priority_operators(mut elements: Vec<CalculationElement>, brackets: &Brackets) -> Vec<CalculationElement> {
        let mut operation_index: usize = 0;

        for (index, element) in elements.iter().enumerate() {
            if element.element_type != ElementType::Operation {
                continue;
            }
            if !CalculationOperation::is_operator_priority(&element.char) {
                continue;
            }
            operation_index = index;
            break;
        }

        if (operation_index == 0) {
            return elements;
        }

        let a = elements.get(operation_index - 1).unwrap();
        let b = elements.get(operation_index + 1).unwrap();
        let operation = elements.get(operation_index).unwrap();
        let calculation_part = CalculationPart::new(a, b, operation);
        let value = finish_calculation(&calculation_part);
        let new_calc_element = CalculationElement::new(ElementType::Number, value.to_string());
        elements.splice(operation_index - 1 .. operation_index + 2, vec![new_calc_element]);

        calculate_all_priority_operators(elements, brackets)
    }

    fn calculate_all_default_operators(mut elements: Vec<CalculationElement>, brackets: &Brackets) -> Vec<CalculationElement> {
        let mut operation_index: usize = 0;

        for (index, element) in elements.iter().enumerate() {
            if element.element_type != ElementType::Operation {
                continue;
            }
            operation_index = index;
            break;
        }

        if (operation_index == 0) {
            return elements;
        }

        let a = elements.get(operation_index - 1).unwrap();
        let b = elements.get(operation_index + 1).unwrap();
        let operation = elements.get(operation_index).unwrap();
        let calculation_part = CalculationPart::new(a, b, operation);
        let value = finish_calculation(&calculation_part);
        let new_calc_element = CalculationElement::new(ElementType::Number, value.to_string());
        elements.splice(operation_index - 1 .. operation_index + 2, vec![new_calc_element]);

        calculate_all_default_operators(elements, brackets)
    }

    #[derive(Clone)]
    pub struct CalculationElement {
        pub element_type: ElementType,
        pub char: String,
    }

    impl CalculationElement {
        pub fn new(element_type: ElementType, char: String) -> CalculationElement {
            CalculationElement {
                element_type, char,
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    enum ElementType {
        Number,
        Operation,
    }

    fn finish_calculation(calculation: &CalculationPart) -> f64 {
        let &a = &calculation.a.char.parse::<f64>().map_err(|_| "Failed to parse 'a' as float").unwrap();
        let &b = &calculation.b.char.parse::<f64>().map_err(|_| "Failed to parse 'b' as float").unwrap();
        let operator = CalculationOperation::operation_from_char(&calculation.operator.char);
        match operator {
            CalculationOperation::Addition => a + b,
            CalculationOperation::Subtraction => a - b,
            CalculationOperation::Multiplication => a * b,
            CalculationOperation::Division => a / b,
            CalculationOperation::Power => f64::powf(a, b),
            CalculationOperation::Null => 0.0,
        }
    }


    fn string_to_calculation(calculation: &str) -> Vec<CalculationPart> {
        let element_collection = Vec::new();

        for char in calculation.chars() {

        }

        element_collection
    }

    pub struct SingleBracket {
    }

    impl SingleBracket {
        pub fn new () -> SingleBracket {
            SingleBracket {}
        }

    }

   pub struct Brackets {
        start_bracket: SingleBracket,
        end_bracket: SingleBracket,
        content: String,
    }

    impl Brackets {
        pub fn new (start_bracket: SingleBracket, end_bracket: SingleBracket, content: String) -> Brackets {
            Brackets{
                start_bracket, end_bracket, content,
            }
        }

        pub fn calculate_bracket(&self) -> f64 {
            let content = &self.content;
            let calculation_elements = string_to_calculation_parts(content);

            calculate_calc_elements(calculation_elements, &self)
        }
    }

    fn is_number(string: &String) -> bool {
        match string.parse::<f64>() {
            Ok(parsed_float) => {
                true
            }
            Err(error) => {
                false
            }
        }
    }

    #[derive(Debug, PartialEq)]
    enum CalculationOperation {
        Addition,
        Multiplication,
        Division,
        Subtraction,
        Power,
        Null,
    }
    impl CalculationOperation {
        fn operation_from_char(string: &str) -> CalculationOperation {
            match string {
                "+" => CalculationOperation::Addition,
                "-" => CalculationOperation::Subtraction,
                "*" => CalculationOperation::Multiplication,
                "/" => CalculationOperation::Division,
                "^" => CalculationOperation::Power,
                _ => CalculationOperation::Null,
            }
        }
        //||
        fn is_operator_priority(string: &str) -> bool {
            "*" == string || "/" == string || "^" == string
        }
    }

}
