# scryptomath for Python

## Why
In more complex Scrypto blueprints you need to calculate your test values against which you want to compare your outputs first.
A good approach to achieve that is to calculate them on a higher level without optimisations staying close to the mathematical specification.

This allows you to validate your mathematical specification more easily, including numerical precision boundaries, by investigating and resolving differences in the calculation results between the Scrypto and Python implementations.

## Features

### Fixed precision decimal types in Python
This library supports you implementing your Python prototype by providing Scrypto decimal types with fixed precision math in Python:
- `Decimal`
- `BalancedDecimal`
- `PreciseDecimal`

Currently the following operations are implemented (more can be added):
- `__abs__`
- `__add__`
- `__sub__`
- `__mul__`
- `__truediv__`  ("normal" division)
- `__pow__`      (exact `pow` result)
- `sqrt()`       (exact `s`)
- `powi()`       (`powi` implementation Scrypto is using to match results and not identical to result of `__pow__`)

However, use this wisely if necessary only because depending on the use case you rather want to calculate your test assertion outputs with very high precision (like 500 decimal places) in Python first. That's what we are doing by default for Ociswap and only use the fixed decimal point types if it is required to match the test values.

Disclaimer: if you are using other operations than the ones above a normal Python `decimal.Decimal` with the full precision you have specified is returned. In that case the return value is not being trimmed down to the fixed decimal places of the Scrypto decimal.

### Code generation for decimal constants in Scrypto

Currently, the macros `dec!` and `pdec!` can't be used for constants in Scrypto.
To workaround that limitation we have implemented Scrypto code generation for initialising decimal constants directly in Scrypto:
```
>>> from scryptomath import Decimal
>>> Decimal(3).powi(4).scrypto
'Decimal(BnumI256::from_digits([7213023705161793536, 4, 0, 0]))'
```

Which can be directly used in your Scrypto blueprint like this:
```
const MY_CONST: Decimal = Decimal(BnumI256::from_digits([7213023705161793536, 4, 0, 0]));
```

Eventually, we assume Scrypto probably will support compile time macros for `dec!` and `pdec!`, but in the meantime this is working very well as a workaround.

## Contribute

If you have any requirements for additional data types just open an issue or pull request. Happy to add more :)