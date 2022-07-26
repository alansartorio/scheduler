use super::collidable::Collidable;
use std::fmt::Display;

use super::time::Time;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    pub start: Time,
    pub end: Time,
}

impl Span {
    pub fn new(start: Time, end: Time) -> Span {
        assert!(start < end);
        Span { start, end }
    }

    pub fn duration(&self) -> u64 {
        self.end - self.start
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.start, self.end)
    }
}

impl Collidable for Span {
    fn collides(&self, other: &Self) -> bool {
        !((self.start < other.start && self.end <= other.start)
            || (self.end > other.end && self.start >= other.end))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collide_spans() {
        let time1 = Time::new(1, 0);
        let time2 = Time::new(2, 0);
        let time3 = Time::new(3, 0);
        let time4 = Time::new(4, 0);
        let time5 = Time::new(5, 0);
        assert!(!Span::collides(
            &Span::new(time1, time2),
            &Span::new(time2, time3)
        ),);
        assert!(Span::collides(
            &Span::new(time1, time3),
            &Span::new(time2, time3)
        ),);
        assert!(Span::collides(
            &Span::new(time1, time4),
            &Span::new(time2, time3)
        ),);
        assert!(!Span::collides(
            &Span::new(time4, time5),
            &Span::new(time3, time4)
        ),);
    }
}
