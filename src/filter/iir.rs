use rustfft::num_complex::Complex;

use super::*;

#[derive(Clone, Copy)]
pub struct IIR;

impl Response for IIR {}

impl IIR {
    pub fn butterworth() -> FilterDesigner<IIR, Butterworth>{
        FilterDesigner {
            impulse_response: IIR,
            kind: Butterworth,
        }
    }
    pub fn elliptic() -> FilterDesigner<IIR, Elliptic>{
        FilterDesigner {
            impulse_response: IIR,
            kind: Elliptic,
        }
    }
    pub fn chebyshev() -> FilterDesigner<IIR, Chebyshev>{
        FilterDesigner {
            impulse_response: IIR,
            kind: Chebyshev,
        }
    }
    pub fn bessel() -> FilterDesigner<IIR, Bessel>{
        FilterDesigner {
            impulse_response: IIR,
            kind: Bessel,
        }
    }
}

impl Filter<IIR, LOWPASS, Butterworth> {
    pub fn coefficients(&self) {
        let cutoff_frequency: f64 = self.cutoff_frequency.into();
        let pi = std::f64::consts::PI;
        let omega_c: f64 = 2.0 * pi * cutoff_frequency;
        let poles: Vec<Complex<_>> = {
            (0..self.order).map(|k| {
                let theta = (pi / 2.0) + (pi * (2.0 * k as f64 + 1.0) / (2.0 * self.order as f64));
                Complex::new(omega_c * theta.cos(), omega_c * theta.sin()) 
            }).collect()
        };

        // let gain = omega_c.pow(2) / (s.pow(2) + s*2.0.sqrt() * omega_c + omega_c.pow(2));

    }
}

