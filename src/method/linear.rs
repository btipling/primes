use method::method::Method;
pub struct Linear {}

impl<'a> Linear {
    pub fn new() -> &'static Linear {
        &Linear { }
    }
}

impl<'a> Method for Linear {
    fn is_prime(&self, n: i64) -> bool {
        let mut i = 2;
        while i < n {
            if n % i == 0 {
                return false;
            }
            i += 1;
        }
        true
    }

    fn name(&self) -> &'static str {
        "linear"
    }
}
