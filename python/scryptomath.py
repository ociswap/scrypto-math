from abc import abstractmethod
from typing import Union
import decimal


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

    def __abs__(self) -> decimal.Decimal:
        return self._cast(super().__abs__())

    def __add__(self, __value: decimal.Decimal):
        return self._cast(super().__add__(__value))

    def __sub__(self, __value: decimal.Decimal):
        return self._cast(super().__sub__(__value))

    def __mul__(self, __value: decimal.Decimal):
        return self._cast(super().__mul__(__value))

    def __truediv__(self, __value: decimal.Decimal):
        return self._cast(super().__truediv__(__value))

    def __pow__(self, __value: decimal.Decimal):
        return self._cast(super().__pow__(__value))

    def sqrt(self, context: decimal.Context | None = None):
        return self._cast(super().sqrt(context))

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
