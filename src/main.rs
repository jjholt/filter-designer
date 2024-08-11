extern crate rustfft;
mod filter;
mod frequency;

use filter::iir::IIR;
use frequency::IntoFrequency;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Errors {
    NegativeFrequency,
}
fn main() {
    IIR::butterworth().low_pass(5, 5.0.khz());
}
