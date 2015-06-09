extern crate rgsl;
extern crate gnuplot;

mod interpolator;

use interpolator::Interpolator;
use gnuplot::{Figure, Caption, Color};

fn main() {
    let xa = &[1_f64, 2., 3.];
    let ya = &[6_f64, 2., 9.];
    let mut interp = Interpolator::new_polynomial(xa, ya);
    let x = (10..31).map(|x| x as f64 / 10.0).collect::<Vec<_>>();
    let y = x.iter().map(|x| interp.eval(*x)).collect::<Vec<_>>();
    let mut fg = Figure::new();
    fg.axes2d().lines(x.iter(), y.iter(), &[Caption("Interpolated"), Color("black")]);
    fg.show();
}
