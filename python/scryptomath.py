from abc import abstractmethod
from typing import Union
import decimal
from decimal import localcontext


decimal.getcontext().prec = 500


def _to_decimal_digits(
    number: Union[str, decimal.Decimal], bits: int, decimal_places: int
):
    context = decimal.getcontext()
    context.prec = 500
    full_number = int(decimal.Decimal(number) * 10**decimal_places)
    if full_number > 2 ** (bits - 1) - 1:
        raise ValueError("Value is too large.")
    if full_number < -1 * 2 ** (bits - 1):
        raise ValueError("Value is too small.")
    two_complement = _twos_complement(full_number, bits)
    return list(reversed([int(chunk, 2) for chunk in _chunks(two_complement, 64)]))


def _twos_complement(value: int, bits: int):
    if value < 0:
        value = (1 << bits) + value
    return ("{:0%ib}" % bits).format(value)


def _chunks(lst, n: int):
    for i in range(0, len(lst), n):
        yield lst[i : i + n]


def ceil_to_decimal(number: decimal.Decimal, precision=18) -> decimal.Decimal:
    decimal.getcontext().rounding = decimal.ROUND_CEILING
    return round(number, precision)


def floor_to_decimal(number: decimal.Decimal, precision=18) -> decimal.Decimal:
    decimal.getcontext().rounding = decimal.ROUND_FLOOR
    return round(number, precision)


class FixedBaseDecimal(decimal.Decimal):
    @abstractmethod
    def _cast(self, __value: decimal.Decimal):
        return

    def __abs__(self, *args, **kwargs) -> decimal.Decimal:
        return self._cast(super().__abs__(*args, **kwargs))

    def __add__(self, *args, **kwargs):
        return self._cast(super().__add__(*args, **kwargs))

    def __sub__(self, *args, **kwargs):
        return self._cast(super().__sub__(*args, **kwargs))

    def __mul__(self, *args, **kwargs):
        return self._cast(super().__mul__(*args, **kwargs))

    def __truediv__(self, *args, **kwargs):
        return self._cast(super().__truediv__(*args, **kwargs))

    def __pow__(self, *args, **kwargs):
        return self._cast(super().__pow__(*args, **kwargs))

    def sqrt(self, *args, **kwargs):
        return self._cast(super().sqrt(*args, **kwargs))

    def powi(self, exp: int):
        # use fixed precision rounding in every iteration to match the Scrypto calculation
        one = self.__class__(1)
        if exp < 0:
            sub = one / self
            return sub.powi(exp * -1)
        if exp == 0:
            return one
        if exp == 1:
            return self
        sub = self * self
        if exp % 2 == 1:
            return self * sub.powi((exp - 1) // 2)
        return sub.powi(exp // 2)


class ScryptoBaseDecimal(FixedBaseDecimal):
    def to_digits(self):
        return _to_decimal_digits(
            self, bits=self.bits, decimal_places=self.decimal_places
        )

    def __repr__(self) -> str:
        return f"{self.__class__.__name__}('{self}')"

    @classmethod
    def __new__(cls, __value, context: decimal.Context | None = None):
        value = super().__new__(__value, context)
        if (
            isinstance(value, decimal.Decimal)
            and value.as_tuple().exponent >= -cls.decimal_places
        ):
            return value
        return cls._cast(value)

    @classmethod
    def _cast(cls, __value: decimal.Decimal, rounding=decimal.ROUND_FLOOR):
        return cls(
            __value.quantize(decimal.Decimal(10) ** -cls.decimal_places, rounding)
        )

    @property
    def scrypto(self):
        scrypto_type = self.__class__.__name__
        return f"{scrypto_type}(I{self.bits}::from_digits({self.to_digits()}))"


class Decimal(ScryptoBaseDecimal):
    decimal_places = 18
    bits = 192


class PreciseDecimal(ScryptoBaseDecimal):
    decimal_places = 36
    bits = 256

    def ceil_to_decimal(self) -> "Decimal":
        return Decimal._cast(self, decimal.ROUND_CEILING)

    def floor_to_decimal(self) -> "Decimal":
        return Decimal._cast(self)


def relative_error(result: decimal.Decimal, error: decimal.Decimal):
    with localcontext() as context:
        context.prec = 40
        return abs(error) / abs(result)


def error_ln() -> decimal.Decimal:
    """
    Approximation error of `ln` is bound by `2^-58.45 ~ 2.6*10^-18`.

    ```txt
    error_ln = 2^-58.45 ~ 2.6*10^-18
    ```
    """
    with localcontext() as context:
        context.prec = 40
        return decimal.Decimal(2) ** decimal.Decimal("-58.45")


def error_exp(value: decimal.Decimal) -> decimal.Decimal:
    """
    Approxmation error of exp_r(r) is bound by `2^-59 ~ 1.8*10^-18` with reduced argument `r` of `x`.

    ```txt
    e^x = 2^k * exp_r'(r)                         with k determined by the argument reduction
    e^x = 2^k * (exp_r(r) + error_exp_r)
    e^x = 2^k * exp_r(r) + 2^k * error_exp_r
    e^x = 2^k * exp_r(r) + error_exp

    error_exp(value) = 2^k * error_exp_r = 2^k * 2^-59 = 2^(k - 59)
    ```
    """
    with localcontext() as context:
        context.prec = 40
        signed_half = (
            decimal.Decimal("-0.5")
            if value < decimal.Decimal(0)
            else decimal.Decimal("0.5")
        )
        k = int((value / decimal.Decimal(2).ln() + signed_half))
        return decimal.Decimal(2) ** (k - 59)


def error_pow(base: decimal.Decimal, exp: decimal.Decimal) -> decimal.Decimal:
    """
    Calculatiion of `pow` is based on `exp` and `ln`:
    ```txt
    x^y = e^(ln(x) * y)
    ```

    Accounting for approximation errors gives:
    ```txt
    e'^(ln'(x) * y)
    = e'^((ln(x) + error_ln) * y)
    = e'^(ln(x) * y + error_ln * y)
    ```

    Using Taylor expansion we can approximate e^(x+error) for |error| << 1 with: e^(n + error) ~ e^n + e^n * error.
    e'(n) and ln'(n) represent the exponential and logarithmic function with approximation error.

    Even with an unreasonable large exponent like y=10^6, we'd still get:
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
    """
    with localcontext() as context:
        context.prec = 40
        e_exp = base.ln() * exp
        return error_exp(e_exp) + e_exp.exp() * error_ln() * exp
