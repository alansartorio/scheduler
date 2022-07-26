pub trait Combinable {
    fn combine(&self, other: &Self) -> Self;
}
