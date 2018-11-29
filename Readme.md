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