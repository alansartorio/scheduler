pub trait Collidable<Rhs = Self> {
    fn collides(&self, other: &Rhs) -> bool;
}
