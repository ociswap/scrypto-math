use crate::exponential::ExponentialPreciseDecimal;
use crate::logarithm::LogarithmPreciseDecimal;
use num_traits::ToPrimitive;
use radix_engine_common::{
    math::{CheckedMul, Decimal, PreciseDecimal},
    pdec,
};

pub trait PowerDecimal {
    fn pow(&self, exp: Decimal) -> Option<Decimal>;
}

pub trait PowerPreciseDecimal {
    fn pow(&self, exp: PreciseDecimal) -> Option<PreciseDecimal>;
}

impl PowerDecimal for Decimal {
    fn pow(&self, exp: Decimal) -> Option<Decimal> {
        let exp = PreciseDecimal::try_from(exp).ok()?;
        PreciseDecimal::try_from(*self)
            .ok()?
            .pow(exp)
            .and_then(|e| e.try_into().ok())
    }
}

impl PowerPreciseDecimal for PreciseDecimal {
    fn pow(&self, exp: PreciseDecimal) -> Option<PreciseDecimal> {
        // based on https://github.com/rust-lang/libm/blob/master/src/math/pow.rs
        if exp == PreciseDecimal::ZERO {
            return Some(PreciseDecimal::ONE);
        }
        if *self == PreciseDecimal::ONE {
            return Some(PreciseDecimal::ONE);
        }
        if *self == PreciseDecimal::ZERO && exp.is_positive() {
            return Some(PreciseDecimal::ZERO);
        }
        if *self == PreciseDecimal::ZERO && exp.is_negative() {
            return None;
        }
        if exp == PreciseDecimal::ONE {
            return Some(self.clone());
        }
        if exp == pdec!(-1) {
            return Some(PreciseDecimal::ONE / *self);
        }

        if self.is_negative() {
            let exp_is_integer =
                PreciseDecimal(exp.0 / PreciseDecimal::ONE.0 * PreciseDecimal::ONE.0) == exp;
            if !exp_is_integer {
                return None;
            }
            let is_even = (exp.0 / PreciseDecimal::ONE.0).to_i32()? % 2 == 0;
            let pow = (self.checked_abs()?.ln()? * exp).exp();
            if is_even {
                return pow;
            }
            return Some(pdec!(-1) * pow?);
        }

        Some((self.ln()?.checked_mul(exp))?.exp()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use radix_engine_common::dec;

    #[test]
    fn test_pow_exp_zero() {
        assert_eq!(dec!(-2).pow(dec!(0)), Some(dec!(1)));
        assert_eq!(dec!(-1).pow(dec!(0)), Some(dec!(1)));
        assert_eq!(dec!(0).pow(dec!(0)), Some(dec!(1)));
        assert_eq!(dec!(1).pow(dec!(0)), Some(dec!(1)));
        assert_eq!(dec!(2).pow(dec!(0)), Some(dec!(1)));
    }

    #[test]
    fn test_pow_base_one() {
        assert_eq!(dec!(1).pow(dec!(2)), Some(dec!(1)));
        assert_eq!(dec!(1).pow(dec!(-2)), Some(dec!(1)));
    }

    #[test]
    fn test_pow_base_zero() {
        assert_eq!(dec!(0).pow(dec!(-2)), None);
        assert_eq!(dec!(0).pow(dec!(-1)), None);
        assert_eq!(dec!(0).pow(dec!(0)), Some(dec!(1)));
        assert_eq!(dec!(0).pow(dec!(1)), Some(dec!(0)));
        assert_eq!(dec!(0).pow(dec!(2)), Some(dec!(0)));
    }

    #[test]
    fn test_pow_exp_one() {
        assert_eq!(dec!(2).pow(dec!(1)), Some(dec!(2)));
        assert_eq!(dec!(-2).pow(dec!(1)), Some(dec!(-2)));
    }

    #[test]
    fn test_pow_exp_minus_one() {
        assert_eq!(dec!(2).pow(dec!(-1)), Some(dec!("0.5")));
        assert_eq!(dec!(-2).pow(dec!(-1)), Some(dec!("-0.5")));
    }

    #[test]
    fn test_pow_base_negative_exp_integer() {
        assert_eq!(dec!(2).pow(dec!(-2)), Some(dec!("0.25")));
        assert_eq!(dec!(-2).pow(dec!(2)), Some(dec!("4")));
        assert_eq!(dec!(-2).pow(dec!(-2)), Some(dec!("0.25")));
        assert_eq!(dec!(5).pow(dec!(-5)), Some(dec!("0.00032")));
        assert_eq!(
            dec!(-5).pow(dec!(5)),
            Some(dec!("-3125") + dec!("0.000000000000001660"))
        );
        assert_eq!(dec!(-5).pow(dec!(-5)), Some(dec!("-0.00032")));
    }

    #[test]
    fn test_pow_base_negative_exp_non_integer() {
        assert_eq!(dec!("-1.1").pow(dec!("0.00000000000000001")), None);
        assert_eq!(dec!("-3.4").pow(dec!("15.43")), None);
        assert_eq!(dec!("-3.4").pow(dec!("-15.43")), None);
    }

    #[test]
    fn test_pow_base_maximum_exp_non_integer() {
        assert_eq!(dec!("-1.1").pow(dec!("0.00000000000000001")), None);
        assert_eq!(dec!("-3.4").pow(dec!("15.43")), None);
        assert_eq!(dec!("-3.4").pow(dec!("-15.43")), None);
    }

    #[test]
    fn test_pow_smallest_value() {
        assert_eq!(
            dec!("3.4").pow(dec!("-33.43")),
            Some(dec!("0.000000000000000001"))
        );
    }

    #[test]
    fn test_pow_largest_value() {
        assert_eq!(
            dec!("3.4").pow(dec!("71.43")),
            Some(
                dec!("91947313437872693600354888137039353441.244419982586019069")
                    - dec!("187832408272640032348.012171022248677284")
            )
        );
    }

    #[test]
    fn test_pow_base_minimum() {
        assert_eq!(Decimal::MIN.pow(dec!(3)), None);
        assert_eq!(Decimal::MIN.pow(Decimal::MIN), None);
        assert_eq!(Decimal::MIN.pow(Decimal::MAX), None);
    }

    #[test]
    fn test_pow_base_maximum() {
        assert_eq!(Decimal::MAX.pow(dec!(3)), None);
        assert_eq!(Decimal::MAX.pow(Decimal::MIN), None);
        assert_eq!(Decimal::MAX.pow(Decimal::MAX), None);
    }

    #[test]
    fn test_pow_base_positive_normal() {
        assert_eq!(dec!(2).pow(dec!(2)), Some(dec!(4)));
        assert_eq!(
            dec!("3.4").pow(dec!("15.43")),
            Some(dec!("158752177.142935864260984228") - dec!("0.000000000094162353"))
        );
        assert_eq!(
            dec!("3.4").pow(dec!("-15.43")),
            Some(dec!("0.000000006299126210"))
        );
    }
}
