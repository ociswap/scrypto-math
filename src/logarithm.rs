use num_traits::Zero;
use radix_engine_common::dec;
use radix_engine_common::math::bnum_integer::*;
use radix_engine_common::math::decimal::*;
use radix_engine_common::*;

use crate::balanced_decimal::BalancedDecimal;
use crate::bdec;

const LN2: BalancedDecimal = BalancedDecimal(BnumI256::from_digits([
    4887746961572732391,
    3757558395076474551,
    0,
    0,
]));
const LN10: BalancedDecimal = BalancedDecimal(BnumI256::from_digits([
    701367864101705880,
    12482338800784407930,
    0,
    0,
]));
const SQRT: BalancedDecimal = BalancedDecimal(BnumI256::from_digits([
    3583277511020380144,
    7666467083416870407,
    0,
    0,
]));
const SQRT_HALF: BalancedDecimal = BalancedDecimal(BnumI256::from_digits([
    11015010792364965880,
    3833233541708435203,
    0,
    0,
]));

const LG1: BalancedDecimal = BalancedDecimal(BnumI256::from_digits([
    7480962735871623168,
    3614007241618385227,
    0,
    0,
])); // 6.666666666666735130e-01
const LG2: BalancedDecimal = BalancedDecimal(BnumI256::from_digits([
    3510068727102046208,
    2168404344938993412,
    0,
    0,
])); // 3.999999999940941908e-01
const LG3: BalancedDecimal = BalancedDecimal(BnumI256::from_digits([
    8437904063097995264,
    1548860255744677571,
    0,
    0,
])); // 2.857142874366239149e-01; /* 3FD24924 94229359 */
const LG4: BalancedDecimal = BalancedDecimal(BnumI256::from_digits([
    10113378558716936192,
    1204667790877038313,
    0,
    0,
])); // 2.222219843214978396e-01
const LG5: BalancedDecimal = BalancedDecimal(BnumI256::from_digits([
    4675241628135325696,
    985733422058661494,
    0,
    0,
])); // 1.818357216161805012e-01
const LG6: BalancedDecimal = BalancedDecimal(BnumI256::from_digits([
    866527560993865728,
    830164805128661067,
    0,
    0,
])); // 1.531383769920937332e-01;
const LG7: BalancedDecimal = BalancedDecimal(BnumI256::from_digits([
    13792263540114456576,
    802211953826968189,
    0,
    0,
])); // 1.479819860511658591e-01
pub trait LogarithmDecimal {
    fn ln(&self) -> Option<Decimal>;
    fn log2(&self) -> Option<Decimal>;
    fn log10(&self) -> Option<Decimal>;
    fn log_base(&self, base: Decimal) -> Option<Decimal>;
}

pub trait LogarithmBalancedDecimal {
    fn ln(&self) -> Option<BalancedDecimal>;
    fn log2(&self) -> Option<BalancedDecimal>;
    fn log10(&self) -> Option<BalancedDecimal>;
    fn log_base(&self, base: BalancedDecimal) -> Option<BalancedDecimal>;
}

fn log_reduce_argument(number: BalancedDecimal) -> (i32, BalancedDecimal) {
    let full_integer = number.0 / BalancedDecimal::ONE.0;

    if full_integer.is_zero() {
        if number >= SQRT_HALF {
            return (0, number);
        }
        let k = number.0.leading_zeros() as i32 - SQRT_HALF.0.leading_zeros() as i32;
        let r = number * BalancedDecimal(BalancedDecimal::ONE.0 << BnumI256::from(k));

        if r >= SQRT {
            return (-k, r);
        }

        return (-k - 1, r * dec!(2));
    }

    let k = 255 - full_integer.leading_zeros() as i32; // index highest integer bit
    let r = number / BalancedDecimal(BalancedDecimal::ONE.0 << BnumI256::from(k));

    if r <= SQRT {
        return (k, r);
    }

    return (k + 1, r / dec!(2));
}

impl LogarithmDecimal for Decimal {
    fn ln(&self) -> Option<Decimal> {
        BalancedDecimal::try_from(*self)
            .ok()?
            .ln()
            .map(|log| log.into())
    }

    fn log2(&self) -> Option<Decimal> {
        BalancedDecimal::try_from(*self)
            .ok()?
            .log2()
            .map(|log| log.into())
    }

    fn log10(&self) -> Option<Decimal> {
        BalancedDecimal::try_from(*self)
            .ok()?
            .log10()
            .map(|log| log.into())
    }

    fn log_base(&self, base: Decimal) -> Option<Decimal> {
        let base = BalancedDecimal::try_from(base).ok()?;
        BalancedDecimal::try_from(*self)
            .ok()?
            .log_base(base)
            .map(|log| log.into())
    }
}

impl LogarithmBalancedDecimal for BalancedDecimal {
    fn ln(&self) -> Option<BalancedDecimal> {
        // based on https://github.com/rust-lang/libm/blob/master/src/math/log.rs
        if !self.is_positive() {
            return None;
        }
        let (k, r) = log_reduce_argument(*self);
        // println!("k = {:?}, r = {:?}", k, r);
        // println!("x_n = {:?}", pdec!(2).powi(k.into()) * r);
        // println!("x_o = {:?}", self);

        let f = r - BalancedDecimal::ONE;
        let s = f / (bdec!(2) + f);
        let z = s * s;
        let w = z * z;
        let remez = z * (LG1 + w * (LG3 + w * (LG5 + w * LG7))) + w * (LG2 + w * (LG4 + w * LG6));
        Some(LN2 * k + f - s * (f - remez))
    }

    fn log2(&self) -> Option<BalancedDecimal> {
        Some(self.ln()? / LN2)
    }

    fn log10(&self) -> Option<BalancedDecimal> {
        Some(self.ln()? / LN10)
    }

    fn log_base(&self, base: BalancedDecimal) -> Option<BalancedDecimal> {
        let base_ln = base.ln()?;
        Some(self.ln()? / base_ln)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_ln_positive_number() {
        assert_eq!(
            dec!(10).ln(),
            Some(dec!("2.302585092994045684") - dec!("0.000000000000000001"))
        );
        assert_eq!(
            bdec!(10).ln(),
            Some(
                bdec!("2.30258509299404568401799145468436420760")
                    - bdec!("0.00000000000000000009315192671654058382")
            )
        );
    }

    #[test]
    fn test_ln_e() {
        assert_eq!(
            dec!("2.718281828459045235").ln(),
            Some(dec!(1) - dec!("0.000000000000000001"))
        );
        assert_eq!(
            bdec!("2.71828182845904523536028747135266249775").ln(),
            Some(bdec!(1) - bdec!("0.00000000000000000007225640213908820418"))
        );
    }

    #[test]
    fn test_ln_one() {
        assert_eq!(dec!(1).ln(), Some(dec!(0)));
        assert_eq!(bdec!(1).ln(), Some(bdec!(0)));
    }

    #[test]
    fn test_ln_zero() {
        assert_eq!(dec!(0).ln(), None);
        assert_eq!(bdec!(0).ln(), None);
    }

    #[test]
    fn test_ln_negative_number() {
        assert_eq!(dec!(-1).ln(), None);
        assert_eq!(bdec!(-1).ln(), None);
    }

    #[test]
    fn test_ln_decimal_precision() {
        assert_eq!(
            dec!("1000000000000000000000000000000").ln(),
            Some(dec!("69.077552789821370520"))
        );
        assert_eq!(
            bdec!("1000000000000000000000000000000").ln(),
            Some(
                bdec!("69.07755278982137052053974364053092622803")
                    + bdec!("0.00000000000000000027411142136760615405")
            )
        );
    }

    #[test]
    fn test_ln_smallest_positive() {
        assert_eq!(
            dec!("0.000000000000000001").ln(),
            Some(dec!("-41.446531673892822312"))
        );
        assert_eq!(
            bdec!("0.00000000000000000000000000000000000001").ln(),
            Some(
                bdec!("-87.49823353377373599268367527800583988885")
                    - bdec!("0.00000000000000000002088682154503733556")
            )
        );
    }

    #[test]
    fn test_ln_maximum_possible() {
        assert_eq!(
            Decimal::from(BalancedDecimal::MAX).ln(),
            Some(dec!("89.254297509012317908"))
        );
        assert_eq!(
            BalancedDecimal::MAX.ln(),
            Some(
                bdec!("89.25429750901231790871051569382918497041")
                    - bdec!("0.00000000000000000002088682154503733698")
            )
        );
    }

    #[test]
    fn test_ln_value_too_large() {
        assert_eq!((Decimal::from(BalancedDecimal::MAX) + 1).ln(), None);
        assert_eq!(Decimal::MAX.ln(), None);
    }

    #[test]
    fn test_log_2() {
        assert_eq!(dec!(-1).log2(), None);
        assert_eq!(dec!(0).log2(), None);
        assert_eq!(dec!(1).log2(), Some(dec!(0)));
        assert_eq!(
            dec!("1.5").log2(),
            Some(dec!("0.584962500721156181") - dec!("0.000000000000000001"))
        );
        assert_eq!(dec!(2).log2(), Some(dec!(1)));
        assert_eq!(dec!(10).log2(), Some(dec!("3.321928094887362347")));
    }

    #[test]
    fn test_log_10() {
        assert_eq!(dec!(-1).log10(), None);
        assert_eq!(dec!(0).log10(), None);
        assert_eq!(dec!(1).log10(), Some(dec!(0)));
        assert_eq!(dec!(5).log10(), Some(dec!("0.698970004336018804")));
        assert_eq!(
            dec!(10).log10(),
            Some(dec!(1) - dec!("0.000000000000000001"))
        );
        assert_eq!(dec!(20).log10(), Some(dec!("1.301029995663981195")));
    }

    #[test]
    fn test_log_base() {
        assert_eq!(dec!(-1).log_base(dec!(8)), None);
        assert_eq!(dec!(0).log_base(dec!(8)), None);
        assert_eq!(dec!(1).log_base(dec!(8)), Some(dec!(0)));
        assert_eq!(
            dec!(5).log_base(dec!(8)),
            Some(dec!("0.773976031629120782"))
        );
        assert_eq!(dec!(8).log_base(dec!(8)), Some(dec!(1)));
        assert_eq!(
            dec!(10).log_base(dec!(8)),
            Some(dec!("1.107309364962454115"))
        );
        assert_eq!(
            dec!(20).log_base(dec!(8)),
            Some(dec!("1.440642698295787449"))
        );
        assert_eq!(dec!(8).log_base(Decimal::MAX), None);
    }
}
