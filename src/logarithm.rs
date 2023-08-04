use num_traits::Zero;
use pretty_assertions::assert_eq;
use radix_engine_common::dec;
use radix_engine_common::math::bnum_integer::*;
use radix_engine_common::math::decimal::*;
use radix_engine_common::prelude::PreciseDecimal;
use radix_engine_common::*;

use crate::balanced_decimal::BalancedDecimal;
use crate::bdec;

const LN2HI: BalancedDecimal = BalancedDecimal(BnumI256::from_digits([
    5936224389055119360,
    3757558394042029165,
    0,
    0,
])); //6.93147180369123816490e-01; /* 0x3fe62e42, 0xfee00000 */
const LN2LO: BalancedDecimal = BalancedDecimal(BnumI256::from_digits([
    17380931500609507840,
    1034445385,
    0,
    0,
])); // 1.90821492927058770002e-10; /* 0x3dea39ef, 0x35793c76 */
const HALF: Decimal = Decimal(BnumI256::from_digits([500000000000000000, 0, 0, 0]));
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

/*const LG1: Decimal = Decimal(BnumI256::from_digits([666666666666673513, 0, 0, 0])); // 6.666666666666735130e-01; /* 3FE55555 55555593 */
const LG2: Decimal = Decimal(BnumI256::from_digits([399999999994094190, 0, 0, 0])); // 3.999999999940941908e-01; /* 3FD99999 9997FA04 */
const LG3: Decimal = Decimal(BnumI256::from_digits([285714287436623914, 0, 0, 0])); // 2.857142874366239149e-01; /* 3FD24924 94229359 */
const LG4: Decimal = Decimal(BnumI256::from_digits([222221984321497839, 0, 0, 0])); // 2.222219843214978396e-01; /* 3FCC71C5 1D8E78AF */
const LG5: Decimal = Decimal(BnumI256::from_digits([181835721616180501, 0, 0, 0])); // 1.818357216161805012e-01; /* 3FC74664 96CB03DE */
const LG6: Decimal = Decimal(BnumI256::from_digits([153138376992093733, 0, 0, 0])); // 1.531383769920937332e-01; /* 3FC39A09 D078C69F */
const LG7: Decimal = Decimal(BnumI256::from_digits([147981986051165859, 0, 0, 0])); // 1.479819860511658591e-01; /* 3FC2F112 DF3E5244 */
*/
const LG1: BalancedDecimal = BalancedDecimal(BnumI256::from_digits([
    7480962735871623168,
    3614007241618385227,
    0,
    0,
])); // 6.666666666666735130e-01; /* 3FE55555 55555593 */
const LG2: BalancedDecimal = BalancedDecimal(BnumI256::from_digits([
    3510068727102046208,
    2168404344938993412,
    0,
    0,
])); // 3.999999999940941908e-01; /* 3FD99999 9997FA04 */
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
])); // 2.222219843214978396e-01; /* 3FCC71C5 1D8E78AF */
const LG5: BalancedDecimal = BalancedDecimal(BnumI256::from_digits([
    4675241628135325696,
    985733422058661494,
    0,
    0,
])); // 1.818357216161805012e-01; /* 3FC74664 96CB03DE */
const LG6: BalancedDecimal = BalancedDecimal(BnumI256::from_digits([
    866527560993865728,
    830164805128661067,
    0,
    0,
])); // 1.531383769920937332e-01; /* 3FC39A09 D078C69F */
const LG7: BalancedDecimal = BalancedDecimal(BnumI256::from_digits([
    13792263540114456576,
    802211953826968189,
    0,
    0,
])); // 1.479819860511658591e-01; /* 3FC2F112 DF3E5244 */
pub trait Logarithm {
    fn log(&self) -> Option<Decimal>;
}
fn log_reduce_arguments(number: Decimal) -> (i32, BalancedDecimal) {
    let full_integer = number.0 / Decimal::ONE.0;

    if full_integer.is_zero() {
        let number = BalancedDecimal::try_from(number).unwrap();
        if number >= SQRT_HALF {
            return (0, number);
        }
        let k = number.0.leading_zeros() as i32 - SQRT_HALF.0.leading_zeros() as i32;
        let r = number * Decimal(Decimal::ONE.0 << BnumI256::from(k));

        if r >= SQRT {
            return (-k, r);
        }

        return (-k - 1, r * dec!(2));
    }

    let k = 255 - full_integer.leading_zeros() as i32; // index highest integer bit
    let r = PreciseDecimal::from(number) / Decimal(Decimal::ONE.0 << BnumI256::from(k));
    let r: BalancedDecimal = BalancedDecimal::try_from(r).unwrap();

    if r <= SQRT {
        return (k, r);
    }

    return (k + 1, r / dec!(2));
}

impl Logarithm for Decimal {
    fn log(&self) -> Option<Decimal> {
        // based on https://github.com/rust-lang/libm/blob/master/src/math/log.rs
        if !self.is_positive() {
            return None;
        }
        let (k, r) = log_reduce_arguments(*self);
        println!("k = {:?}, r = {:?}", k, r);
        println!("x_n = {:?}", pdec!(2).powi(k.into()) * r);
        println!("x_o = {:?}", self);

        let f = r - bdec!(1);
        let hfsq = f * f * HALF;
        let s = f / (bdec!(2) + f);
        let z = s * s;
        let w = z * z;
        let t1 = w * (LG2 + w * (LG4 + w * LG6));
        let t2 = z * (LG1 + w * (LG3 + w * (LG5 + w * LG7)));
        let res = t2 + t1;
        let k = BalancedDecimal::from(k);
        Some((s * (hfsq + res) + LN2LO * k - hfsq + f + LN2HI * k).into())
    }
}

#[test]
fn test_log_positive_number() {
    assert_eq!(dec!(10).log(), Some(dec!("2.302585092994045684")));
}

#[test]
fn test_log_one() {
    assert_eq!(dec!(1).log(), Some(dec!(0)));
}

#[test]
fn test_log_zero() {
    assert_eq!(dec!(0).log(), None);
}

#[test]
fn test_log_negative_number() {
    assert_eq!(dec!(-1).log(), None);
}

#[test]
fn test_log_decimal_precision() {
    assert_eq!(
        dec!("1000000000000000000000000000000").log(),
        Some(dec!("69.077552789821370520"))
    );
}

#[test]
fn test_log_smallest_positive() {
    assert_eq!(
        dec!("0.000000000000000001").log(),
        Some(dec!("-41.446531673892822312"))
    );
}

#[test]
fn test_log_maximum() {
    assert_eq!(Decimal::MAX.log(), Some(dec!("135.305999368893231589")));
}
