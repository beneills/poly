mod gcd;
mod polynomial;
mod quotient;

pub use polynomial::Polynomial;
pub use quotient::Quotient;

/// Demo the crate's functionality by performing symbolic differentiation and integration.
pub fn demo() {
    // Allocate p(x) = 3 + 5x + 7x^2
    let p: Polynomial = Polynomial::new(
        [ Quotient::from_int(3),    // x^0
          Quotient::from_int(5),    // x^1
          Quotient::from_int(7),    // x^2
          Quotient::from_int(0) ]); // x^3

    println!("We have a polynomial in Q[x], p(x) = {}",     p);
    println!("We can differentiate it to get p'(x) = {}",   p.differentiate());
    println!("And we can integrate it to get I[p(x)] = {}", p.integrate());
}
