# binary_ok

You can use real and integers numbers, as you wish.

Put this on your cargo.toml file to use this crate:
```
[dependencies]
binary_ok = "0.2.1"
```
If you are not using f32 or f64, just use the u128 versions functions, that way you are able to do some math on the flow.
```
use::binary_ok;

binary_ok::u128_to_binary(number: &u128);
binary_ok::u128_to_decimal(number: &u128);

binary_ok::to_binary(number: &f64);
binary_ok::to_decimal(number: &f64);
```
u128_to_binary() returns a u128 value

u128_to_decimal() return a u128 value

to_binary() returns a bits string (Due to the IEEE 754 standard and the way that Rust handle float numbers, as far as I know, is impossible to convert the string using the [parse() method](https://doc.rust-lang.org/std/string/struct.String.html#method.parse) to f32 or f64 with high precision to decimal places)

to_decimal() returns a f64 decimal number
```
use::binary_ok;

let decimal_number: u128 = 87654884988;
println!("Decimal number for conversion: {}", decimal_number);

let binary_number: u128 = binary_ok::u128_to_binary(decimal_number);
println!("Binary conversion result: {}", binary_number);

let to_decimal_again: u128 = binary_ok::u128_to_decimal(binary_number);
println!("To decimal again: {}", to_decimal_again);
```
Output:

Decimal number for conversion: 87654884988

Binary conversion result: 1010001101000101000110110011001111100

To decimal again: 87654884988
```
let x: f64 = 1110101.0;
let y: f64 = 1001101.11;

let result: String = binary_ok::to_binary(&(binary_ok::to_decimal(&x) + binary_ok::to_decimal(&y)));
println!("{}", result);
```
Output:

11000010.11