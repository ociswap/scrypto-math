use num_traits::ToPrimitive;
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
const HALF_POSITIVE: BalancedDecimal = BalancedDecimal(BnumI256::from_digits([
    343699775700336640,
    2710505431213761085,
    0,
    0,
]));
const HALF_NEGATIVE: BalancedDecimal = BalancedDecimal(BnumI256::from_digits([
    18103044298009214976,
    15736238642495790530,
    18446744073709551615,
    18446744073709551615,
]));
const INVLN2: BalancedDecimal = BalancedDecimal(BnumI256::from_digits([
    14785644574383232046,
    7820865487829388881,
    0,
    0,
]));

const P1: BalancedDecimal = BalancedDecimal(BnumI256::from_digits([
    9815257410705686528,
    903501810404583517,
    0,
    0,
])); // 1.66666666666666019037e-01
const P2: BalancedDecimal = BalancedDecimal(BnumI256::from_digits([
    15655445234027528192,
    18431685710203221679,
    18446744073709551615,
    18446744073709551615,
])); //  -2.77777777770155933842e-03
const P3: BalancedDecimal = BalancedDecimal(BnumI256::from_digits([
    6103176189629636608,
    358532448599637,
    0,
    0,
])); // 6.61375632143793436117e-05
const P4: BalancedDecimal = BalancedDecimal(BnumI256::from_digits([
    15690603967107440640,
    18446735110663206201,
    18446744073709551615,
    18446744073709551615,
])); // -1.65339022054652515390e-06
const P5: BalancedDecimal = BalancedDecimal(BnumI256::from_digits([
    10758981003257543680,
    224328845270,
    0,
    0,
])); //4.13813679705723846039e-08

pub trait ExponentialDecimal {
    fn exp(&self) -> Option<Decimal>;
}

pub trait ExponentialBalancedDecimal {
    fn exp(&self) -> Option<BalancedDecimal>;
}

impl ExponentialDecimal for Decimal {
    fn exp(&self) -> Option<Decimal> {
        if self < &dec!(-42) {
            return Some(Decimal::ZERO);
        }

        if self > &dec!(88) {
            return None;
        }
        BalancedDecimal::try_from(*self)
            .ok()?
            .exp()
            .map(|e| e.into())
    }
}

impl ExponentialBalancedDecimal for BalancedDecimal {
    fn exp(&self) -> Option<BalancedDecimal> {
        // based on https://github.com/rust-lang/libm/blob/master/src/math/exp.rs
        if self.is_zero() {
            return Some(BalancedDecimal::ONE);
        }
        if self < &bdec!(-88) {
            return Some(BalancedDecimal::ZERO);
        }
        if self > &bdec!(88) {
            return None;
        }

        let signed_half = if self.is_negative() {
            HALF_NEGATIVE
        } else {
            HALF_POSITIVE
        };

        // r = x - floor(x/ln(2) +- 0.5) * ln(2)
        // https://www.wolframalpha.com/input?i=x+-+floor%28x%2Fln%282%29+%2B+0.5%29+*+ln%282%29
        let k = INVLN2 * *self + signed_half;
        let k: i32 = (k.0 / BalancedDecimal::ONE.0).to_i32().unwrap();
        let r = *self - LN2 * k;

        // println!("k = {:?}, r = {:?}", k, r);
        // println!("x_n = {:?}", LN2 * k + r);
        // println!("x_o = {:?}", self);

        let rr = r * r;
        let c = r - rr * (P1 + rr * (P2 + rr * (P3 + rr * (P4 + rr * P5))));
        let exp_r = BalancedDecimal::ONE + r + (r * c) / (dec!(2) - c);

        let two_pow_k = if self.is_negative() {
            BalancedDecimal(BalancedDecimal::ONE.0 >> k.abs().into())
        } else {
            BalancedDecimal(BalancedDecimal::ONE.0 << k.into()) // k <= 127
        };
        Some(two_pow_k * exp_r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_exponent_positive() {
        assert_eq!(dec!("0.1").exp(), Some(dec!("1.105170918075647624")));
        assert_eq!(
            bdec!("0.1").exp(),
            Some(
                bdec!("1.10517091807564762481170782649024666822")
                    + bdec!("0.00000000000000000007184816985596142021")
            )
        );
        assert_eq!(
            dec!(1).exp(),
            Some(dec!("2.718281828459045235") - dec!("0.000000000000000001"))
        );
        assert_eq!(
            bdec!(1).exp(),
            Some(
                bdec!("2.71828182845904523536028747135266249775")
                    - bdec!("0.00000000000000000056697581981633858323")
            )
        );
        assert_eq!(
            dec!(2).exp(),
            Some(dec!("7.389056098930650227") - dec!("0.000000000000000001"))
        );
        assert_eq!(
            bdec!(2).exp(),
            Some(
                bdec!("7.38905609893065022723042746057500781318")
                    - bdec!("0.00000000000000000049348300468610545638")
            )
        );
        assert_eq!(
            dec!(5).exp(),
            Some(dec!("148.413159102576603421") - dec!("0.000000000000000014"))
        );
        assert_eq!(
            bdec!(5).exp(),
            Some(
                bdec!("148.41315910257660342111558004055227962348")
                    - bdec!("0.00000000000000001343302197722530682860")
            )
        );
        assert_eq!(
            dec!(10).exp(),
            Some(dec!("22026.465794806716516957") - dec!("0.000000000000005071"))
        );
        assert_eq!(
            bdec!(10).exp(),
            Some(
                bdec!("22026.46579480671651695790064528424436635351")
                    - bdec!("0.00000000000000507172344337393935374039")
            )
        );
    }

    #[test]
    fn test_exponent_negative() {
        assert_eq!(dec!("-0.1").exp(), Some(dec!("0.904837418035959573")));
        assert_eq!(
            bdec!("-0.1").exp(),
            Some(
                bdec!("0.90483741803595957316424905944643662119")
                    - bdec!("0.00000000000000000005885564521858091519")
            )
        );
        assert_eq!(dec!(-1).exp(), Some(dec!("0.367879441171442321")));
        assert_eq!(
            bdec!(-1).exp(),
            Some(
                bdec!("0.36787944117144232159552377016146086744")
                    + bdec!("0.00000000000000000007020286164488250299")
            )
        );
        assert_eq!(dec!(-2).exp(), Some(dec!("0.135335283236612691")));
        assert_eq!(
            bdec!(-2).exp(),
            Some(
                bdec!("0.13533528323661269189399949497248440340")
                    + bdec!("0.00000000000000000000905932412046340746")
            )
        );
        assert_eq!(dec!(-5).exp(), Some(dec!("0.006737946999085467")));
        assert_eq!(
            bdec!(-5).exp(),
            Some(
                bdec!("0.00673794699908546709663604842314842424")
                    + bdec!("0.00000000000000000000059663563307729027")
            )
        );
        assert_eq!(dec!(-10).exp(), Some(dec!("0.000045399929762484")));
        assert_eq!(
            bdec!(-10).exp(),
            Some(
                bdec!("0.00004539992976248485153559151556055061")
                    + bdec!("0.00000000000000000000001002862199768050")
            )
        );
    }

    #[test]
    fn test_exponent_zero() {
        assert_eq!(dec!(0).exp(), Some(dec!(1)));
        assert_eq!(bdec!(0).exp(), Some(bdec!(1)));
    }

    #[test]
    fn test_exponent_large_value() {
        assert_eq!(
            dec!(80).exp(),
            Some(
                dec!("55406223843935100525711733958316612.92485672883268532")
                    - dec!("8563021306004937.882349426621353341")
            )
        );
        assert_eq!(
            bdec!(80).exp(),
            Some(
                bdec!("55406223843935100525711733958316612.92485672883268532287030018828204570044")
                    - bdec!("8563021306004937.88234942662135334307788474904963249596")
            )
        );
    }

    #[test]
    fn test_exponent_small_value() {
        assert_eq!(dec!(-30).exp(), Some(dec!("0.000000000000093576")));
        assert_eq!(
            bdec!(-60).exp(),
            Some(bdec!("0.00000000000000000000000000875651076269"))
        );
    }

    #[test]
    fn test_exponent_smallest_value() {
        assert_eq!(dec!(-41).exp(), Some(dec!("0.000000000000000001")));
        assert_eq!(
            bdec!(-87).exp(),
            Some(bdec!("0.00000000000000000000000000000000000001"))
        );
    }

    #[test]
    fn test_exponent_largest_value() {
        assert_eq!(
            dec!(88).exp(),
            Some(
                dec!("165163625499400185552832979626485876706.962884200004481388")
                    - dec!("1247592587037918071.347548424821445671")
            )
        );
        assert_eq!(
            bdec!(88).exp(),
            Some(
                bdec!("165163625499400185552832979626485876706.96288420000448138888115075308155590848")
                 - bdec!("1247592587037918071.34754842482144567130490168358910580928")
            )
        );
    }

    #[test]
    fn test_exponent_value_too_small() {
        assert_eq!(dec!(-42).exp(), Some(dec!(0)));
        assert_eq!(bdec!(-88).exp(), Some(bdec!(0)));
    }

    #[test]
    fn test_exponent_value_too_large() {
        assert_eq!(dec!(89).exp(), None);
        assert_eq!(bdec!(89).exp(), None);
    }

    #[test]
    fn test_exponent_negative_min() {
        assert_eq!(Decimal::MIN.exp(), Some(dec!(0)));
        assert_eq!(BalancedDecimal::MIN.exp(), Some(bdec!(0)));
    }

    #[test]
    fn test_exponent_positive_max() {
        assert_eq!(Decimal::MAX.exp(), None);
        assert_eq!(BalancedDecimal::MAX.exp(), None);
    }
}
