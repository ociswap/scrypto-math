use num_traits::ToPrimitive;
use radix_engine_common::math::bnum_integer::I256;
use radix_engine_common::math::{Decimal, PreciseDecimal};
use radix_engine_common::{pdec, dec};

const LN2: PreciseDecimal = PreciseDecimal(I256::from_digits([
    9456716947207598648,
    37575583950764745,
    0,
    0,
]));
const HALF_POSITIVE: PreciseDecimal = PreciseDecimal(I256::from_digits([
    15683169460410122240,
    27105054312137610,
    0,
    0,
]));
const HALF_NEGATIVE: PreciseDecimal = PreciseDecimal(I256::from_digits([
    2763574613299429376,
    18419639019397414005,
    18446744073709551615,
    18446744073709551615,
]));
const INVLN2: PreciseDecimal = PreciseDecimal(I256::from_digits([
    15089719145448569129,
    78208654878293888,
    0,
    0,
]));

const P1: PreciseDecimal = PreciseDecimal(I256::from_digits([
    3234099066637680640,
    9035018104045835,
    0,
    0,
])); // 1.66666666666666019037e-01
const P2: PreciseDecimal = PreciseDecimal(I256::from_digits([
    11778003218777292800,
    18446593490074488316,
    18446744073709551615,
    18446744073709551615,
])); //  -2.77777777770155933842e-03
const P3: PreciseDecimal = PreciseDecimal(I256::from_digits([
    6886327069168830464,
    3585324485996,
    0,
    0,
])); // 6.61375632143793436117e-05
const P4: PreciseDecimal = PreciseDecimal(I256::from_digits([
    15836638502324193280,
    18446743984079088161,
    18446744073709551615,
    18446744073709551615,
])); // -1.65339022054652515390e-06
const P5: PreciseDecimal =
    PreciseDecimal(I256::from_digits([13020310661629261568, 2243288452, 0, 0])); //4.13813679705723846039e-08

pub trait ExponentialDecimal {
    fn exp(&self) -> Option<Decimal>;
}

pub trait ExponentialPreciseDecimal {
    fn exp(&self) -> Option<PreciseDecimal>;
}

impl ExponentialDecimal for Decimal {
    fn exp(&self) -> Option<Decimal> {
        if self < &dec!(-42) {
            return Some(Decimal::ZERO);
        }
        if self > &dec!(90) {
            return None;
        }
        PreciseDecimal::try_from(*self)
            .ok()?
            .exp()
            .and_then(|e| e.try_into().ok())
    }
}

impl ExponentialPreciseDecimal for PreciseDecimal {
    fn exp(&self) -> Option<PreciseDecimal> {
        // based on https://github.com/rust-lang/libm/blob/master/src/math/exp.rs
        if self.is_zero() {
            return Some(PreciseDecimal::ONE);
        }
        if self < &pdec!(-82) {
            return Some(PreciseDecimal::ZERO);
        }
        if self > &pdec!(93) {
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
        let k: i32 = (k.0 / PreciseDecimal::ONE.0).to_i32().unwrap();
        let r = *self - LN2 * k;

        // println!("k = {:?}, r = {:?}", k, r);
        // println!("x_n = {:?}", LN2 * k + r);
        // println!("x_o = {:?}", self);

        let rr = r * r;
        let c = r - rr * (P1 + rr * (P2 + rr * (P3 + rr * (P4 + rr * P5))));
        let exp_r = PreciseDecimal::ONE + r + (r * c) / (dec!(2) - c);

        let two_pow_k = if self.is_negative() {
            PreciseDecimal(PreciseDecimal::ONE.0 >> k.abs() as u32)
        } else {
            PreciseDecimal(PreciseDecimal::ONE.0 << k as u32) // k <= 130
        };
        Some(two_pow_k * exp_r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_constants() {
        assert_eq!(LN2, pdec!("0.693147180559945309417232121458176568"));
        assert_eq!(HALF_POSITIVE, pdec!("0.5"));
        assert_eq!(HALF_NEGATIVE, pdec!("-0.5"));
        assert_eq!(INVLN2, pdec!("1.442695040888963407359924681001892137"));
        assert_eq!(P1, pdec!("0.166666666666666019037"));
        assert_eq!(P2, pdec!("-0.00277777777770155933842"));
        assert_eq!(P3, pdec!("0.0000661375632143793436117"));
        assert_eq!(P4, pdec!("-0.00000165339022054652515390"));
        assert_eq!(P5, pdec!("0.0000000413813679705723846039"));
    }

    #[test]
    fn test_exponent_positive() {
        assert_eq!(dec!("0.1").exp(), Some(dec!("1.105170918075647624")));
        assert_eq!(
            pdec!("0.1").exp(),
            Some(
                pdec!("1.105170918075647624811707826490246668")
                    + pdec!("0.000000000000000000073249221022502114")
            )
        );
        assert_eq!(
            dec!(1).exp(),
            Some(dec!("2.718281828459045235") - dec!("0.000000000000000001"))
        );
        assert_eq!(
            pdec!(1).exp(),
            Some(
                pdec!("2.718281828459045235360287471352662497")
                    - pdec!("0.000000000000000000506600695098127761")
            )
        );
        assert_eq!(
            dec!(2).exp(),
            Some(dec!("7.389056098930650227") - dec!("0.000000000000000001"))
        );
        assert_eq!(
            pdec!(2).exp(),
            Some(
                pdec!("7.389056098930650227230427460575007813")
                    - pdec!("0.000000000000000000502826567049772189")
            )
        );
        assert_eq!(
            dec!(5).exp(),
            Some(dec!("148.413159102576603421") - dec!("0.000000000000000013"))
        );
        assert_eq!(
            pdec!(5).exp(),
            Some(
                pdec!("148.413159102576603421115580040552279623")
                    - pdec!("0.000000000000000012819743652169222343")
            )
        );
        assert_eq!(
            dec!(10).exp(),
            Some(dec!("22026.465794806716516957") - dec!("0.000000000000004654"))
        );
        assert_eq!(
            pdec!(10).exp(),
            Some(
                pdec!("22026.465794806716516957900645284244366353")
                    - pdec!("0.000000000000004654463413405594362897")
            )
        );
    }

    #[test]
    fn test_exponent_negative() {
        assert_eq!(dec!("-0.1").exp(), Some(dec!("0.904837418035959573")));
        assert_eq!(
            pdec!("-0.1").exp(),
            Some(
                pdec!("0.904837418035959573164249059446436621")
                    - pdec!("0.000000000000000000059971389890128697")
            )
        );
        assert_eq!(dec!(-1).exp(), Some(dec!("0.367879441171442321")));
        assert_eq!(
            pdec!(-1).exp(),
            Some(
                pdec!("0.367879441171442321595523770161460867")
                   + pdec!("0.000000000000000000068560948558969987")
            )
        );
        assert_eq!(dec!(-2).exp(), Some(dec!("0.135335283236612691")));
        assert_eq!(
            pdec!(-2).exp(),
            Some(
                pdec!("0.135335283236612691893999494972484403")
                  + pdec!("0.000000000000000000009209589825745512")
            )
        );
        assert_eq!(dec!(-5).exp(), Some(dec!("0.006737946999085467")));
        assert_eq!(
            pdec!(-5).exp(),
            Some(
                pdec!("0.006737946999085467096636048423148424")
                    + pdec!("0.000000000000000000000582015461381543")
            )
        );
        assert_eq!(dec!(-10).exp(), Some(dec!("0.000045399929762484")));
        assert_eq!(
            pdec!(-10).exp(),
            Some(
                pdec!("0.000045399929762484851535591515560550")
                   + pdec!("0.000000000000000000000009593564125049")
            )
        );
    }

    #[test]
    fn test_exponent_zero() {
        assert_eq!(dec!(0).exp(), Some(dec!(1)));
        assert_eq!(pdec!(0).exp(), Some(pdec!(1)));
    }

    #[test]
    fn test_exponent_large_value() {
        assert_eq!(
            dec!(80).exp(),
            Some(
                dec!("55406223843935100525711733958316612.924856728832685322")
                    - dec!("8411471907589238.909955041056771071")
            )
        );
        assert_eq!(
            pdec!(80).exp(),
            Some(
                pdec!("55406223843935100525711733958316612.924856728832685322870300188282045700")
                    - pdec!("8411471907589238.909955041056771071656863326999790852")
            )
        );
    }

    #[test]
    fn test_exponent_small_value() {
        assert_eq!(dec!(-30).exp(), Some(dec!("0.000000000000093576")));
        assert_eq!(
            pdec!(-60).exp(),
            Some(
                pdec!("0.000000000000000000000000008756510762")
                    - pdec!("0.000000000000000000000000000000000001")
            )
        );
    }

    #[test]
    fn test_exponent_smallest_value() {
        assert_eq!(dec!(-41).exp(), Some(dec!("0.000000000000000001")));
        assert_eq!(
            pdec!(-82).exp(),
            Some(pdec!("0.000000000000000000000000000000000002"))
        );
    }

    #[test]
    fn test_exponent_largest_value() {
        assert_eq!(
            dec!(90).exp(),
            Some(
                dec!("1220403294317840802002710035136369753970.746421099767546244")
                    - dec!("62783923595896661921.607585533121275855")
            )
        );
        assert_eq!(
            pdec!(93).exp(),
            Some(
                pdec!("24512455429200857855527729431109153423487.564149646906095458338836041506325882")
                 + pdec!("673513250279373616826.005878421400866518707554460260006534")
            )
        );
    }

    #[test]
    fn test_exponent_value_too_small() {
        assert_eq!(dec!(-42).exp(), Some(dec!(0)));
        assert_eq!(pdec!(-83).exp(), Some(pdec!(0)));
    }

    #[test]
    fn test_exponent_value_too_large() {
        assert_eq!(dec!(91).exp(), None);
        assert_eq!(pdec!(94).exp(), None);
    }

    #[test]
    fn test_exponent_negative_min() {
        assert_eq!(Decimal::MIN.exp(), Some(dec!(0)));
        assert_eq!(PreciseDecimal::MIN.exp(), Some(pdec!(0)));
    }

    #[test]
    fn test_exponent_positive_max() {
        assert_eq!(Decimal::MAX.exp(), None);
        assert_eq!(PreciseDecimal::MAX.exp(), None);
    }
}
