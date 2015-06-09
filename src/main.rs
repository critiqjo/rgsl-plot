extern crate rgsl;

mod interpolator;

use interpolator::Interpolator;

fn main() {
    let xa = &[1_f64, 2., 3.];
    let ya = &[6_f64, 2., 9.];
    let mut interp_test = Interpolator::new_linear(xa, ya);
    println!("interpolated: (1.5, {}), (2.5, {})", interp_test.eval(1.5), interp_test.eval(2.5));
}
