extern crate rustfft;
mod filter;
mod frequency;

use filter::iir::IIR;
use frequency::IntoFrequency;

fn main() {
    let my_filter = IIR::butterworth()
        .low_pass(5, 5.0.MHz());
}
