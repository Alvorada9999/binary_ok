pub fn to_binary(number: &f64) -> f64 {
    
    let mut number_as_integer: u128 = number.trunc().to_string().parse().unwrap();
    let mut bits_string = String::from("");

    //The aboves convert the integer part of the number to binary and puts it on the bits_string
    while number_as_integer >= 1 {
        bits_string.push_str(&(number_as_integer % 2).to_string());
        number_as_integer = number_as_integer / 2;
    }
    bits_string = bits_string.chars().rev().collect::<String>();
    //----------------------------------
    
    //The aboves convert the fractional part of the number to binary if it is bigger than 0.0
    if number.fract() != 0.0 {
        if number.trunc() == 0.0 {
            bits_string.push_str("0");
        }
        bits_string.push_str(".");

        let mut number_fractional_part: f64 = number.fract();

        let mut denominator: f64 = 2.0;
        for _x in 0..10 {
            if number_fractional_part < 1.0/denominator {
                bits_string.push_str("0");
            } else if number_fractional_part > 1.0/denominator {
                bits_string.push_str("1");
                number_fractional_part -= 1.0/denominator;
            } else if number_fractional_part == 1.0/denominator {
                bits_string.push_str("1");
                break;
            }
            denominator *= 2.0;
        }
    }
    //----------------------------------
    return bits_string.parse::<f64>().unwrap();
}

pub fn to_decimal(number: &f64) -> f64 {

    let bits_string: &str = &number.to_string();
    let reverse_bits_string: &str = &bits_string.chars().rev().collect::<String>();
    let bits_string_iterator = bits_string.split("");
    let reverse_bits_string_iterator = reverse_bits_string.split("");
    let mut decimal_number_result: f64 = 0.0;

    //The part aboves convert the integer part of the number to binary
    for i in bits_string_iterator {
        match i {
            "1" => decimal_number_result = decimal_number_result * 2.0 + 1.0,
            "0" => decimal_number_result = decimal_number_result * 2.0,
            "." => break,
            _ => ()
        }
    }
    //----------------------------------

    //The aboves convert the fractional part of the number to decimal if it is bigger than 0.0
    if number.fract() != 0.0 {
        let mut denominator: f64 = 2.0;
        for j in reverse_bits_string_iterator {
            match j {
                "1" => {
                    decimal_number_result += 1.0/denominator;
                    denominator *= 2.0;
                },
                "0" => denominator *= 2.0,
                "." => break,
                _ => ()
            }
        }
    }
    //----------------------------------
    decimal_number_result
}