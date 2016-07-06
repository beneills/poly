use std::fmt;
use quotient::Quotient;
use quotient::ZERO;

/// One less that the maximum possible polynomial degree.
const TOTAL_COEFFICIENTS: usize = 4;

/// A polynomial in one indeterminate in Q[x], of maximum degree TOTAL_COEFFICIENTS - 1.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Polynomial {
    coefficients: [Quotient; TOTAL_COEFFICIENTS]
}

impl Polynomial {
    /// Constructs a new Polynomial from an array of coefficients.
    pub fn new(coefficients: [Quotient; TOTAL_COEFFICIENTS]) -> Polynomial {
        Polynomial {
            coefficients: coefficients
        }
    }

    /// Obvious symbolic differentiation w.r.t. the single indeterminate.
    pub fn differentiate(&self) -> Polynomial {
        let mut new_coefficients: [Quotient; TOTAL_COEFFICIENTS] = [ZERO; TOTAL_COEFFICIENTS];

        for degree in 0..TOTAL_COEFFICIENTS-1 {
            new_coefficients[degree] = self.coefficients[degree + 1] * Quotient::from_int((degree + 1) as u64);
        }

        Polynomial::new(new_coefficients)
    }

    /// Obvious symbolic integration w.r.t. the single indeterminate.
    ///
    /// # Panics
    ///
    /// Will panic if the resultant polynomial cannot fit into a Polynomial representation,
    /// i.e. if the highest order coefficient of the original is non-zero.
    ///
    pub fn integrate(&self) -> Polynomial {
        assert!(ZERO == self.coefficients[TOTAL_COEFFICIENTS - 1]);

        let mut new_coefficients: [Quotient; TOTAL_COEFFICIENTS] = [ZERO; TOTAL_COEFFICIENTS];

        for degree in 1..TOTAL_COEFFICIENTS {
            new_coefficients[degree] = self.coefficients[degree - 1] / Quotient::from_int(degree as u64);
        }

        Polynomial::new(new_coefficients)
    }
}

/// Format a polynomial as a string, with indeterminate x.
///
/// # Examples
///
/// "2 + 3x + 5x^2 + 7x^3"
///
impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}x + {}x^2 + {}x^3", self.coefficients[0], self.coefficients[1], self.coefficients[2], self.coefficients[3])
    }
}

#[test]
fn test_polynomial() {
    let p: Polynomial = Polynomial::new([Quotient::from_int(3), Quotient::from_int(5), Quotient::from_int(7), Quotient::from_int(0)]);
    let p_derivative: Polynomial = Polynomial::new([Quotient::from_int(5), Quotient::from_int(14), Quotient::from_int(0), Quotient::from_int(0)]);
    let p_integral: Polynomial = Polynomial::new([Quotient::from_int(0), Quotient::from_int(3), Quotient::new(5, 2, true), Quotient::new(7, 3, true)]);

    assert_eq!(p_derivative, p.differentiate());
    assert_eq!(p_integral, p.integrate());
}
