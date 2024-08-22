extern crate base_streamer;

use base_streamer::usr_lib_prelude::*;
use std::f64::consts::PI;

usrlib_boilerplate!();

/// Linear function:
///     a: slope
///     b: offset
#[usr_fn_f64]
pub struct MyLinFn {
    a: f64,
    b: f64,
}
impl Calc<f64> for MyLinFn {
    fn calc(&self, t_arr: &ArrayView1<f64>, mut res_arr: ArrayViewMut1<f64>) {
        res_arr.zip_mut_with(t_arr, |res, &t| {
            *res = self.a * t + self.b
        });
    }
}

/// Sine function:
///     amp - amplitude (in Volts)
///     freq - linear frequency (in Hz)
///     phase - absolute phase (in radians)
///     offs - offset (in Volts)
#[usr_fn_f64(amp, freq, phase=0.0, offs=0.0)]
pub struct MySine {
    amp: f64,
    freq: f64,
    phase: f64,
    offs: f64,
}
impl Calc<f64> for MySine {
    fn calc(&self, t_arr: &ArrayView1<f64>, mut x_arr: ArrayViewMut1<f64>) {
        x_arr.zip_mut_with(t_arr, |res, &t| {
            *res = self.offs + self.amp * f64::sin(2.0*PI * self.freq * t + self.phase)
        });
    }
}

/// Boolean constant:
///     val - value
#[usr_fn_bool]
pub struct MyBoolConst {
    val: bool
}
impl Calc<bool> for MyBoolConst {
    fn calc(&self, _t_arr: &ArrayView1<f64>, mut res_arr: ArrayViewMut1<bool>) {
        res_arr.fill(self.val)
    }
}
