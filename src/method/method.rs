
pub trait Method {
    fn is_prime(&self, i64) -> bool;
    fn name(&self) -> &'static str;
}
