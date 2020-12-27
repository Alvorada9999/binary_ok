# binary_ok

You can use real and integers numbers, as you wish.

Put this on your cargo.toml file to use this crate:
```
[dependencies]
binary_ok = "0.1.1"
```
```
use::binary_ok;

binary_ok::to_binary(number: &f64);
binary_ok::to_decimal(number: &f64);
```
to_binary() returns a bits string

to_decimal() returns a f64 decimal number
```
let x: f64 = 1110101.0;
let y: f64 = 1001101.11;

let result: String = binary_ok::to_binary(&(binary_ok::to_decimal(&x) + binary_ok::to_decimal(&y)));
println!("{}", result);
```
Output = 11000010.11