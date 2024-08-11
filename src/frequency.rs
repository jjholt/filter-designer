#[derive(Clone, Copy)]
pub enum Frequency <T>{
    Hz(T),
    MHz(T),
    GHz(T),
}

#[allow(non_snake_case)]
pub trait IntoFrequency <T>{
    fn Hz(self) -> Frequency<T>; 
    fn MHz(self) -> Frequency<T>;
    fn GHz(self) -> Frequency<T>;
}

#[allow(non_snake_case)]
impl <T> IntoFrequency<T> for T where T: PartialOrd + Default {
    fn Hz(self) -> Frequency<T> {
        panic_if_negative(&self);
        Frequency::Hz(self)
    }

    fn MHz(self) -> Frequency<T> {
        panic_if_negative(&self);
        Frequency::MHz(self)
    }

    fn GHz(self) -> Frequency<T> {
        panic_if_negative(&self);
        Frequency::GHz(self)
    }
}

impl From<Frequency<f64>> for f64 {
    fn from(value: Frequency<f64>) -> Self {
        match value {
            Frequency::Hz(f) => f,
            Frequency::MHz(f) => f*1000.0,
            Frequency::GHz(f) => f*1_000_000.0,
        }
    }
}

fn panic_if_negative <T: PartialOrd+Default> (val: &T) {
    if val < &T::default() {
        panic!("Frequency must be positive")
    }
}
