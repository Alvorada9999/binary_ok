# binary_ok

Put this on your cargo.toml file to use this crate:
```
[dependencies]
binary_ok = "0.1.0"
```
use::binary_ok;

binary_ok::to_binary(number: &f64);
binary_ok::to_decimal(number: &f64);
```

You will have a f64 as return.
```
let x: f64 = 1110101.0;
let y: f64 = 1001101.11;

let result: f64 = to_binary(to_decimal(x) + to_decimal(y));
```