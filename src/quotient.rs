use gcd::gcd;

use std::fmt;

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

pub static ZERO: Quotient = Quotient { numerator: 0, denominator: 1, positive: true };

/// A mathematical quotient (or "fraction"), living in the field Q.
///
/// Always represented internally in lowest terms, with zero positive.
///
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Quotient {
    numerator: u64,
    denominator: u64,
    positive: bool
}

impl Quotient {
    /// Constructs a new quotient, given a dividend, divisor and sign.
    ///
    /// # Panics
    ///
    /// Will panic if divisor == 0.
    ///
    pub fn new(dividend: u64, divisor: u64, positive: bool) -> Quotient {
        assert!(0 != divisor);

        let gcd = gcd(dividend, divisor);

        let (numerator, denominator) = match gcd {
            0 => (0, 1),
            _ => (dividend / gcd, divisor / gcd)
        };

        Quotient {
            numerator: numerator,
            denominator: denominator,
            positive: match numerator { 0 => true, _ => positive }
        }
    }

    /// Constructs a quotient from a u64.
    pub fn from_int(i: u64) -> Quotient {
        Quotient::new(i, 1, true)
    }

    /// Returns the additive inverse of this quotient.
    pub fn negative(&self) -> Quotient {
        Quotient::new(self.numerator, self.denominator, ! self.positive)
    }

    /// Returns the multiplicative inverse of this quotient.
    pub fn inverse(&self) -> Quotient {
        assert!(0 != self.numerator);

        Quotient::new(self.denominator, self.numerator, self.positive)
    }

    /// Asserts that the internal representation is correctly normalized.
    ///
    /// # Panics
    ///
    /// Panics if the internal representation is invalid.
    ///
    #[allow(dead_code)]
    fn assert_valid(&self) {
        if 0 == self.numerator {
            assert_eq!(1, self.denominator);
            assert_eq!(true, self.positive);
        }

        assert_eq!(1, gcd(self.numerator, self.denominator))
    }
}

/// Obvious addition of quotients.
impl Add for Quotient {
    type Output = Quotient;

    fn add(self, _rhs: Quotient) -> Quotient {
        let same_sign = self.positive == _rhs.positive;

        let lhs_numerator = self.numerator * _rhs.denominator;
        let rhs_numerator = self.denominator * _rhs.numerator;
        let common_denominator = self.denominator * _rhs.denominator;

        let (numerator, positive) = match same_sign {
            true => (lhs_numerator + rhs_numerator, self.positive),
            false => match rhs_numerator <= lhs_numerator {
                true => (lhs_numerator - rhs_numerator, self.positive),
                false => (rhs_numerator - lhs_numerator, ! self.positive)
            }
        };

        Quotient::new(numerator, common_denominator, positive)
    }
}

/// Obvious subtraction of quotients.
impl Sub for Quotient {
    type Output = Quotient;

    fn sub(self, _rhs: Quotient) -> Quotient {
        self + _rhs.negative()
    }
}

/// Obvious multiplication of quotients.
impl Mul for Quotient {
    type Output = Quotient;

    fn mul(self, _rhs: Quotient) -> Quotient {
        let positive = self.positive == _rhs.positive;

        Quotient::new(self.numerator * _rhs.numerator, self.denominator * _rhs.denominator, positive)
    }
}

/// Obvious division of quotients.
impl Div for Quotient {
    type Output = Quotient;

    fn div(self, _rhs: Quotient) -> Quotient {
        self * _rhs.inverse()
    }
}

/// Obvious addition of quotients.
///
/// # Examples
///
/// "2/3"
/// "-2/3"
/// "0"
/// "2"
///
impl fmt::Display for Quotient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = match self.positive { true => "", false => "-" };
        let long_bottom = format!("/{}", self.denominator);
        let bottom = match self.denominator { 1 => "", _ =>  &*long_bottom };

        write!(f, "{}{}{}", sign, self.numerator, bottom)
    }
}

#[test]
fn test_quotient() {
    let zero = Quotient { numerator: 0, denominator: 1, positive: true };
    assert_eq!(zero, ZERO);
    assert_eq!(zero, Quotient::new(0, 1, true));
    assert_eq!(zero, Quotient::new(0, 2, true));
    assert_eq!(zero, Quotient::new(0, 1, false));
    assert_eq!(zero, zero * zero);

    let one = Quotient { numerator: 1, denominator: 1, positive: true };
    assert_eq!(one, Quotient::new(1, 1, true));
    assert_eq!(one, Quotient::new(6, 6, true));
    assert_eq!(one, one * one);

    let negative_one = Quotient { numerator: 1, denominator: 1, positive: false };
    assert_eq!(negative_one, Quotient::new(1, 1, false));
    assert_eq!(negative_one, Quotient::new(2, 2, false));
    assert_eq!(negative_one, negative_one * one);
    assert_eq!(negative_one, one * negative_one);

    let two_fifths = Quotient { numerator: 2, denominator: 5, positive: true };
    assert_eq!(two_fifths, Quotient::new(2, 5, true));
    assert_eq!(two_fifths, Quotient::new(4, 10, true));
    assert_eq!(two_fifths, zero + two_fifths);
    assert_eq!(two_fifths, one * two_fifths);
    assert_eq!(two_fifths, two_fifths * one);

    let four_fifths = Quotient { numerator: 4, denominator: 5, positive: true };
    assert_eq!(four_fifths, two_fifths + two_fifths);

    let negative_two_fifths = Quotient { numerator: 2, denominator: 5, positive: false };
    assert_eq!(zero, two_fifths + negative_two_fifths);
    assert_eq!(two_fifths, four_fifths + negative_two_fifths);

    assert_eq!(zero, one * zero);
    assert_eq!(zero, zero * one);
    assert_eq!(one, four_fifths * four_fifths + (one - two_fifths) * (one - two_fifths));
}
