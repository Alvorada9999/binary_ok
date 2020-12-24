pub fn to_binary(mut number: f64) -> String {

    let mut number_integer_part: u128 = number.trunc().to_string().parse().unwrap();
    let mut bits_string = String::from("");

    if number % 2.0 == 0.0 {
        while number_integer_part >= 1 {
            bits_string.push_str(&(number_integer_part % 2).to_string());
            number_integer_part = number_integer_part / 2;
        }
        bits_string = bits_string.chars().rev().collect::<String>();
    } else {
        if number_integer_part == 0 {
            bits_string.push_str("0");
        }
        while number_integer_part >= 1 {
            bits_string.push_str(&(number_integer_part % 2).to_string());
            number_integer_part = number_integer_part / 2;
        }
        bits_string = bits_string.chars().rev().collect::<String>();
        
        let mut float_part_for_conversion: u128;

        if (number * 2.0) - (number * 2.0).trunc() != 0.0 {
        bits_string.push_str(".");
            while (number * 2.0) - (number * 2.0).trunc() != 0.0 {
                number *= 2.0;
            }
            float_part_for_conversion = (number * 2.0).to_string().parse::<u128>().unwrap();
        } else {
            float_part_for_conversion = 0;
        }
        let mut float_part_binary_string = String::from("");
        while float_part_for_conversion >= 1 {
            float_part_binary_string.push_str(&(float_part_for_conversion % 2).to_string());
            float_part_for_conversion = float_part_for_conversion / 2;
        }
        float_part_binary_string = float_part_binary_string.chars().rev().collect::<String>();
        bits_string.push_str(&float_part_binary_string);
    }
    return bits_string;
}