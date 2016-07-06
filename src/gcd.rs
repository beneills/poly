/// Computes gcd(a, b).
///
/// An implementation of Euclid's Algorithm
///
pub fn gcd(a: u64, b: u64) -> u64 {
    // let {larger, smaller} = {a, b} s.t. larger >= smaller
    let (mut larger, mut smaller) = if a < b { (b, a) } else { (a, b) };

    // special case where one of the numbers is zero
    if 0 == smaller { return 0; }

    while 0 != smaller {
        let temp = smaller;

        // Set new smaller to the remainder
        smaller = larger % smaller;

        // Set new larger to old smaller
        larger = temp;
    }

    // smaller is now zero; return the penultimate remainder
    larger
}

#[test]
fn test_gcd() {
    assert_eq!(0, gcd(0, 0));
    assert_eq!(0, gcd(0, 1));
    assert_eq!(0, gcd(1, 0));
    assert_eq!(1, gcd(1, 1));
    assert_eq!(1, gcd(1, 90));
    assert_eq!(3, gcd(3, 6));
    assert_eq!(3, gcd(6, 9));
    assert_eq!(3, gcd(15, 39));
}
