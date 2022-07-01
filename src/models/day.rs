use std::cmp::Ordering;

use itertools::Itertools;

use super::{collidable::Collidable, combinable::Combinable, task::Task};

#[derive(Debug, Clone)]
pub struct Day<T> {
    pub tasks: Vec<Task<T>>,
    pub has_collisions: bool,
}

//#[ext]
//impl<T: Ord, I> I
//where
//I: Iterator<Item = T>,
//{
//fn is_monotonic_increasing(mut self) -> bool {
//if let Some(mut last) = self.next() {
//for item in self {
//if !(last < item) {
//return false;
//}
//last = item;
//}
//}
//true
//}
//}

impl<T> Day<T> {
    pub fn new(mut tasks: Vec<Task<T>>) -> Day<T> {
        //assert!(tasks.iter().map(|task| task.span).is_monotonic_increasing());
        tasks.sort_by(|a, b| (&a).span.cmp(&b.span));
        let mut day = Day {
            tasks,
            has_collisions: false,
        };
        day.has_collisions = day.has_collisions();
        day
    }

    pub fn empty() -> Day<T> {
        Self::new(vec![])
    }

    fn has_collisions(&self) -> bool {
        for (task1, task2) in self.tasks.iter().tuple_windows() {
            if task1.span.collides(&task2.span) {
                return true;
            }
        }
        false
    }
}

impl<T: Clone> Combinable for Day<T> {
    fn combine(&self, other: &Self) -> Self {
        let merged: Vec<Task<T>> = self
            .tasks
            .iter()
            .cloned()
            .merge_by(other.tasks.clone(), |a, b| a.span.cmp(&b.span) == Ordering::Less)
            .collect();
        Self::new(merged)
    }
}

impl<T> Collidable for Day<T> {
    fn collides(&self, other: &Self) -> bool {
        if self.has_collisions || other.has_collisions {
            return true;
        }

        let merged: Vec<&Task<T>> = self
            .tasks
            .iter()
            .merge_by(&other.tasks, |a, b| a.span.cmp(&b.span) == Ordering::Less)
            .collect();

        for (t1, t2) in merged.iter().tuple_windows() {
            if t1.span.collides(&t2.span) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::span::Span;
    use crate::models::time::Time;

    //#[test]
    //fn monotonic_tests() {
    //assert_eq!(vec![2, 1, 3, 4].iter().is_monotonic_increasing(), false);
    //assert_eq!(vec![1, 2, 3, 4].iter().is_monotonic_increasing(), true);
    //assert_eq!(vec![2, 2, 3, 4].iter().is_monotonic_increasing(), false);
    //assert_eq!(
    //vec![1, 2, 3, 4, 5, 6, 5, 7, 8, 9]
    //.iter()
    //.is_monotonic_increasing(),
    //false
    //);
    //assert_eq!(
    //vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 9]
    //.iter()
    //.is_monotonic_increasing(),
    //false
    //);
    //assert_eq!(
    //vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    //.iter()
    //.is_monotonic_increasing(),
    //true
    //);
    //}

    #[test]
    fn day_collision() {
        let times = (0..10).map(|i| Time::new(i, 0)).collect::<Vec<Time>>();
        let _span1 = Span::new(times[0], times[1]);
        let _span2 = Span::new(times[1], times[2]);
        let _span4 = Span::new(times[2], times[3]);
        let _span5 = Span::new(times[3], times[4]);
        let _span7 = Span::new(times[6], times[7]);
        let _span3 = Span::new(times[1], times[3]);
        let _span6 = Span::new(times[2], times[4]);
        assert_eq!(
            Day::new(vec![Task::new(_span1, ()), Task::new(_span2, ())]).has_collisions(),
            false
        );
        assert_eq!(
            Day::new(vec![Task::new(_span1, ()), Task::new(_span3, ())]).has_collisions(),
            false
        );
        assert_eq!(
            Day::new(vec![Task::new(_span2, ()), Task::new(_span3, ())]).has_collisions(),
            true
        );
        assert_eq!(
            Day::new(vec![
                Task::new(_span1, ()),
                Task::new(_span2, ()),
                Task::new(_span4, ()),
                Task::new(_span5, ())
            ])
            .has_collisions(),
            false
        );
        assert_eq!(
            Day::new(vec![
                Task::new(_span1, ()),
                Task::new(_span2, ()),
                Task::new(_span6, ()),
                Task::new(_span5, ())
            ])
            .has_collisions(),
            true
        );
    }
}
