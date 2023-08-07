# scrypto_math

## Why
Radix Scrypto currently only supports two decimal types `Decimal` and `PreciseDecimal` which are also lacking more advanced mathematical operations like `exp`, `log` or `pow`.

`scrypto_math` aims to provide an alternative until these functionalities are provided upstream. The ultimate goal of `scrypto_math` however is to make itself obsolete.

## Usage
Add `scrypto_math` to your depdencies in the `Cargo.toml` of your Scrypto blueprint.
```rust
[dependencies]
scrypto_math = { git = "https://github.com/ociswap/scrypto-math", branch = "main" }
```

## Featues
### BalancedDecimal
`BalancedDecimal` is a Scrypto decimal type which has 38 decimal places, but using the same 256-bit integers under the hood as `Decimal` (which only has 18 decimal places) for equal performance and fee costs. This is achieved by shifting the decimal places and therefore lowering the maxium and minimum values.

Keep in mind that `BalancedDecimal` is not supported by `ScryptoSbor` and therefore can not be used anywhere where supporting `ScryptoSbor` is required. This includes:
- component state variables (also indirect usage e.g. via `KeyValueStore`)
- NFT fields
- public function/method parameters and return values

For full support a full-stack integration is needed for which we have published this pull request https://github.com/radixdlt/radixdlt-scrypto/pull/1295

In practice this is not a large limitation and you can work around that by using `BalancedDecimal` only for internal calculation and convert from/to `Decimal` / `PreciseDecimal` at entering or leaving your function depending on your use case and needs. Storing/passing `BalancedDecimal` as `PreciseDecimal` is the best option since allows for lossless conversions in both directions. Also a mixing can make sense. For example your input value is a `Decimal` (e.g. a token amount), but you want to calculate and store the result in a higher precision.

Converting from and to `BalancedDecimal` should be fairly cheap and doesn't introduce much overhead compared to the actual calculations.

### Conversion
- `Decimal` to `BalancedDecimal` using the `TryFrom` trait can fail due to overflow.
- `PreciseDecimal` to `BalancedDecimal` using the `TryFrom` trait can fail due overflow and truncates decimal places.
- `BalancedDecimal` to `Decimal` using the `From` trait truncates decimal places.
- `BalancedDecimal` to `PreciseDecimal` using the `From` trait is lossless.


#### Usage
Import the module:
```rust
use scrypto_math::balanced_decimal::*;
```

Usage with `Decimal`
```rust
let input: Decimal = dec!("1.2"):
let amount: BalancedDecimal = BalancedDecimal::try_from(amount).expect("Value too large.");
let output: Decimal = amount.into()
```

Usage with `PreciseDecimal`
```rust
let input: PreciseDecimal = pdec!("1.2"):
let amount: BalancedDecimal = BalancedDecimal::try_from(amount).expect("Value too large.");
let output: PreciseDecimal = amount.into()
```

You can see a full blueprint example including tests here [BalancedDecimalDemo](examples/balanced_decimal/src/lib.rs).


### Exponential Function
The exponential function is provided for `Decimal` and `BalancedDecimal` with a polynomial approximation error lower than ~18 significant digits.
Background: the final result is calculated via `exp(x) = 2**k * R(r)` and the approximation `R(r)` is bound by an maximum error of `2**-59` (~ 18 decimal places).

For `Decimal`:
```rust
let exp: Option<Decimal> = dec!(4).exp();
```

For `BalancedDecimal`:
```rust
let exp: Option<BalancedDecimal> = bdec!(4).exp();
```

You can see a full blueprint example including tests here [AdvancedMathDemo](examples/advanced_math/src/lib.rs).

### Logarithm Function
Logarithm is available for `Decimal` and `BalancedDecimal` with a maximum polynomial approximation error bound by `2**-58.45` (~ 18 decimal places).

For `Decimal`:
```rust
let ln: Option<Decimal> = dec!(2).ln();
let log2: Option<Decimal> = dec!(3).log2();
let log10: Option<Decimal> = dec!(4).log10();
let log8: Option<Decimal> = dec!(5).log_base(base: dec!(8));
```

For `BalancedDecimal`:
```rust
let ln: Option<BalancedDecimal> = bdec!(2).ln();
let log2: Option<BalancedDecimal> = bdec!(3).log2();
let log10: Option<BalancedDecimal> = bdec!(4).log10();
let log8: Option<BalancedDecimal> = bdec!(5).log_base(base: bdec!(8));
```

You can see a full blueprint example including tests here [AdvancedMathDemo](examples/advanced_math/src/lib.rs).

### Power Function
The power function internally uses both `exp` and `ln` and also covers various special cases like `0**0` or `-2**3`.

For `Decimal`:
```rust
let pow: Option<Decimal> = dec!("3.14").pow("-14.12");
```

For `BalancedDecimal`:
```rust
let pow: Option<BalancedDecimal> = bdec!("3.14").pow("-45.97");
```

You can see a full blueprint example including tests here [AdvancedMathDemo](examples/advanced_math/src/lib.rs).

## Contributions
We are happy to collaborate and review and merge pull requests :)

## Disclaimer
Though covered by an extensive test suite, use at your own risk.
