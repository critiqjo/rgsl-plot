use rgsl::interpolation;
use rgsl::types::{InterpAccel, Interp, InterpType};

pub struct Interpolator<'a> {
    xa: &'a[f64],
    ya: &'a[f64],
    interp: Interp,
    i_acc: InterpAccel,
}

#[allow(dead_code)]
impl<'a> Interpolator<'a> {
    pub fn new(xa: &'a[f64], ya: &'a[f64], i_type: &InterpType) -> Interpolator<'a> {
        if xa.len() != ya.len() {
            panic!("Data arrays should be of the same size; given sizes: {}, {}", xa.len(), ya.len());
        }
        let size = xa.len();
        let i_acc = InterpAccel::new();
        if let Some(interp) = Interp::new(i_type, size as u64) {
            interp.init(xa, ya);
            Interpolator { xa: xa, ya: ya, interp: interp, i_acc: i_acc }
        } else {
            panic!("Unable to create Interp object.")
        }
    }
    pub fn new_linear(xa: &'a[f64], ya: &'a[f64]) -> Interpolator<'a> {
        Interpolator::new(xa, ya, &InterpType::linear())
    }
    pub fn new_polynomial(xa: &'a[f64], ya: &'a[f64]) -> Interpolator<'a> {
        Interpolator::new(xa, ya, &InterpType::polynomial())
    }
    pub fn new_cspline(xa: &'a[f64], ya: &'a[f64]) -> Interpolator<'a> {
        Interpolator::new(xa, ya, &InterpType::cspline())
    }
    pub fn new_cspline_periodic(xa: &'a[f64], ya: &'a[f64]) -> Interpolator<'a> {
        Interpolator::new(xa, ya, &InterpType::cspline_periodic())
    }
    pub fn new_akima(xa: &'a[f64], ya: &'a[f64]) -> Interpolator<'a> {
        Interpolator::new(xa, ya, &InterpType::akima())
    }
    pub fn new_akima_periodic(xa: &'a[f64], ya: &'a[f64]) -> Interpolator<'a> {
        Interpolator::new(xa, ya, &InterpType::akima_periodic())
    }
    pub fn eval(&mut self, x: f64) -> f64 {
        interpolation::eval(&self.interp, self.xa, self.ya, x, &mut self.i_acc)
    }
    pub fn eval_deriv(&mut self, x: f64) -> f64 {
        interpolation::eval_deriv(&self.interp, self.xa, self.ya, x, &mut self.i_acc)
    }
    pub fn eval_deriv2(&mut self, x: f64) -> f64 {
        interpolation::eval_deriv2(&self.interp, self.xa, self.ya, x, &mut self.i_acc)
    }
    pub fn eval_integ(&mut self, a: f64, b: f64) -> f64 {
        interpolation::eval_integ(&self.interp, self.xa, self.ya, a, b, &mut self.i_acc)
    }
}
