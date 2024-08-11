pub mod fir;
pub mod iir;

use crate::frequency::Hz;
use num_traits::Float;

pub trait Response: Clone + Copy {}
pub struct LOWPASS;
pub struct HIGHPASS;
pub struct BANDPASS;
pub struct BANDSTOP;
pub struct Butterworth;
pub struct Elliptic;
pub struct Chebyshev;
pub struct Bessel;

pub struct FilterDesigner<R, K> {
    impulse_response: R,
    kind: K
}

pub struct Filter<RESPONSE, PASS, KIND> {
    impulse_response: RESPONSE,
    pass: PASS,
    kind: KIND,
    order: usize,
    cutoff_frequency: Hz<f64>
}

impl<R, K> FilterDesigner<R, K> {
    /// Creates a low pass filter
    pub fn low_pass(self, order: usize, cutoff_frequency: Hz<f64>) -> Filter<R, LOWPASS, K> {
        Filter {
            impulse_response: self.impulse_response,
            kind: self.kind,
            pass: LOWPASS,
            order,
            cutoff_frequency,
        }
    }
    /// Creates a high pass filter
    pub fn high_pass(self, order: usize, cutoff_frequency: Hz<f64>) -> Filter<R, HIGHPASS, K> {
        Filter {
            impulse_response: self.impulse_response,
            kind: self.kind,
            pass: HIGHPASS,
            order,
            cutoff_frequency,
        }
    }
    /// Sets the filter to band pass
    pub fn band_pass(self, order: u32, range: (Hz<f64>,Hz<f64>)) -> Filter<R, BANDPASS, K> {
        todo!()
    }
    /// Sets the filter to band stop
    pub fn band_stop(self, order: u32, range: (Hz<f64>, Hz<f64>)) -> Filter<R, BANDSTOP, K> {
        todo!()
    }
}
