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

## Contributions
We are happy to collaborate and review and merge pull requests :)

## Disclaimer
Though covered by an extensive test suite, use at your own risk.
