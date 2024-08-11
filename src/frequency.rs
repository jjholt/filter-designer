use num_traits::Float;

use crate::Errors;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hz<T: Float>(T);

impl<T: Float> Hz<T> {
    fn new(val: T) -> Result<Hz<T>, Errors> {
        if val < T::zero() {
            return Err(Errors::NegativeFrequency);
        }
        Ok(Hz(val))
    }
}

impl <T: Float> TryFrom<f64> for Hz<T> {
    type Error = Errors;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
            Hz::new(T::from(value).unwrap())
    }
}

pub trait TryIntoFrequency<T: Float> {
    /// Tries to create Hz from number in Hz
    fn try_hz(self) -> Result<Hz<T>, Errors>;
    /// Tries to create Hz from number in KHz
    fn try_khz(self) -> Result<Hz<T>, Errors>;
    /// Tries to create Hz from number in MHz
    fn try_mhz(self) -> Result<Hz<T>, Errors>;
    /// Tries to create Hz from number in GHz
    fn try_ghz(self) -> Result<Hz<T>, Errors>;
}


pub trait IntoFrequency<T: Float> {
    /// From Hz. Panics if frequency is negative
    fn hz(self) -> Hz<T>;
    /// From KHz. Panics if frequency is negative
    fn khz(self) -> Hz<T>;
    /// From MHz. Panics if frequency is negative
    fn mhz(self) -> Hz<T>;
    /// From GHz. Panics if frequency is negative
    fn ghz(self) -> Hz<T>;
}

impl<T: Float> IntoFrequency<T> for T {
    fn hz(self) -> Hz<T> {
        Hz::new(self).unwrap()
    }

    fn khz(self) -> Hz<T> {
        Hz::new(self * T::from(1000).unwrap()).unwrap()
    }

    fn mhz(self) -> Hz<T> {
        Hz::new(self * T::from(1_000_000).unwrap()).unwrap()
    }

    fn ghz(self) -> Hz<T> {
        Hz::new(self * T::from(1_000_000_000).unwrap()).unwrap()
    }
}

impl<T: Float> TryIntoFrequency<T> for T {
    fn try_hz(self) -> Result<Hz<T>, Errors> {
        Hz::new(self)
    }

    fn try_khz(self) -> Result<Hz<T>, Errors> {
        Hz::new(self * T::from(1000).unwrap())
    }

    fn try_mhz(self) -> Result<Hz<T>, Errors> {
        Hz::new(self * T::from(1_000_000).unwrap())
    }

    fn try_ghz(self) -> Result<Hz<T>, Errors> {
        Hz::new(self * T::from(1_000_000_000).unwrap())
    }
}

impl<T: Float + Into<f64>> From<Hz<T>> for f64 {
    fn from(value: Hz<T>) -> Self {
        value.0.into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tries_conversions() {
        assert_eq!(1.0.khz(), Hz(1_000.0));
        assert_eq!(1.0.hz(), Hz(1.0));
        assert_eq!( (-1.0).try_mhz(), Err(Errors::NegativeFrequency));
    }

    #[test]
    fn fails_for_negative_frequencies() {
        assert_eq!(1.0.mhz(), Hz(1_000_000.0));
        let res = std::panic::catch_unwind(|| (-1.0).mhz());
        assert!(res.is_err());
    }
    
    
}
