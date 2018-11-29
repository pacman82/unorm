Unorm
=====

An unsigned normalized real number type, able to represent numbers from zero to one (inclusive).

Usage
-----

```rust
use unorm::Unorm;

let probability_a = Unorm::from_rational(2,3);
let probability_b = Unorm::from_rational(1,3);
let probability_ab = probability_a * probability_b;
```

Why you may not want to use it
------------------------------

Multiplication of `Unorm` with `Unorm` is slow compared with `f64`. Many features you may want are
probably missing. Contributions are welcome though.

Why you may want to use it
--------------------------

Originally developed to represent probabilities in a small pet project of mine. Adding up `Unorm`
within a probability density function is fast and avoids some numeric issues.