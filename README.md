# scrypto_math

## Why
Radix Scrypto currently is lacking more advanced mathematical operations like `exp`, `ln` or `pow`.

`scrypto_math` aims to provide an alternative until these functionalities are provided upstream. The ultimate goal of `scrypto_math` however is to make itself obsolete.

## Usage
Add `scrypto_math` to your depdencies in the `Cargo.toml` of your Scrypto blueprint.
```rust
[dependencies]
scrypto_math = { git = "https://github.com/ociswap/scrypto-math", tag = "v0.6.0" }
```
Import the module:
```rust
use scrypto_math::*;
```

When using mathematical functions like `exp`, `ln`, or `pow`, it's important to account for their approximation errors in your Scrypto code:

- `ln` has a constant maximum error of ~2.6×10⁻¹⁸ (2⁻⁵⁸·⁴⁵)
- `exp` and `pow` have variable maximum errors that depend on the input values
- For `exp(x)`, the error grows exponentially with x
- For `pow(x,y)`, the error depends on both the base x and exponent y

To handle these errors safely:

1. Analyze the possible input ranges for your specific use case
2. Use the Python `scryptomath` library's error estimation functions to calculate maximum errors
3. Add appropriate safety margins by rounding in the direction that favors your component
4. Consider using `PreciseDecimal` for intermediate calculations to minimize error propagation

## Featues

### Exponential Function
The exponential function is provided for `Decimal` and `PreciseDecimal`.

For `Decimal`:
```rust
let exp: Option<Decimal> = dec!(4).exp();
```

For `PreciseDecimal`:
```rust
let exp: Option<PreciseDecimal> = pdec!(4).exp();
```

You can see a full blueprint example including tests here [AdvancedMathDemo](examples/advanced_math/src/lib.rs).

#### Error Estimation
The Approxmation error of `exp_r(r)` is bound by `2^-59 ~ 1.8*10^-18` with reduced argument `r` of `x`.

```txt
e^x = 2^k * exp_r'(r)                         with k determined by the argument reduction
e^x = 2^k * (exp_r(r) + error_exp_r)
e^x = 2^k * exp_r(r) + 2^k * error_exp_r
e^x = 2^k * exp_r(r) + error_exp
```
Resulting in:
```txt
error_exp(value) = 2^k * error_exp_r = 2^k * 2^-59 = 2^(k - 59)
```

Overall this provides an approximation error lower than ~ 18 significant digits. However, the error can overflow to the next digits, meaning this is no guarantee.
Only the maximum error can be guaranteed, but not the significant digits.

The Python library `scryptomath` provides the function [error_exp](python/scryptomath.py) to estimate the maximum error for a specific value.

### Logarithm Function
Logarithm is available for `Decimal` and `PreciseDecimal`. with a maximum polynomial approximation error bound by `2**-58.45` (~ `2.6*10**-18`).

For `Decimal`:
```rust
let ln: Option<Decimal> = dec!(2).ln();
let log2: Option<Decimal> = dec!(3).log2();
let log10: Option<Decimal> = dec!(4).log10();
let log8: Option<Decimal> = dec!(5).log_base(base: dec!(8));
```

For `PreciseDecimal`:
```rust
let ln: Option<PreciseDecimal> = pdec!(2).ln();
let log2: Option<PreciseDecimal> = pdec!(3).log2();
let log10: Option<PreciseDecimal> = pdec!(4).log10();
let log8: Option<PreciseDecimal> = pdec!(5).log_base(base: pdec!(8));
```

You can see a full blueprint example including tests here [AdvancedMathDemo](examples/advanced_math/src/lib.rs).

#### Error Estimation
The maximum polynomial approximation error is bound by the constant `2**-58.45` (~ `2.6*10**-18`).

The Python library `scryptomath` provides the function [error_ln](python/scryptomath.py) giving the maximum error.

### Power Function
The power function internally uses both `exp` and `ln` and also covers various special cases like `0**0` or `-2**3`.

For `Decimal`:
```rust
let pow: Option<Decimal> = dec!("3.14").pow("-14.12");
```

For `PreciseDecimal`:
```rust
let pow: Option<PreciseDecimal> = pdec!("3.14").pow("-45.97");
```

You can see a full blueprint example including tests here [AdvancedMathDemo](examples/advanced_math/src/lib.rs).

#### Error Estimation
Calculation of `pow` is based on `exp` and `ln`:
```txt
x^y = e^(ln(x) * y)
```

Accounting for approximation errors gives:
```txt
e'^(ln'(x) * y)
= e'^((ln(x) + error_ln) * y)
= e'^(ln(x) * y + error_ln * y)
```

Using Taylor expansion we can approximate `e^(x+error)` for `|error| << 1 with: e^(n + error) ~ e^n + e^n * error`.
`e'(n)` and `ln'(n)` represent the exponential and logarithmic function with approximation error.

Even with an unreasonable large exponent like `y=10^6`, we'd still get:
```txt
error_ln * y ≈ (3×10^-18) * 10^6 = 3×10^-12
```
which is much smaller than one.

Allowing to separate the error term:
```txt
e'^(ln(x) * y + error_ln * y)
~ e'^(ln(x) * y) + e^(ln(x) * y) * error_ln * y
= e^(ln(x) * y) + error_exp(ln(x) * y) + e^(ln(x) * y) * error_ln * y
= e^(ln(x) * y) + error_pow(x, y)
```

Resulting in:
```txt
error_pow(x, y) = error_exp(ln(x) * y) + e^(ln(x) * y) * error_ln * y
```

The Python library `scryptomath` provides the function [error_pow](python/scryptomath.py) to estimate the maximum error for specific values.

## Contributions
We are happy to collaborate and review and merge pull requests :)

## Disclaimer
Though covered by an extensive test suite, use at your own risk.
