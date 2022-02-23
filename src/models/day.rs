use super::{collidable::Collidable, task::Task};
//use extend::ext;

pub struct Day<T> {
    tasks: Vec<Task<T>>,
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
        Day { tasks }
    }

    pub fn empty() -> Day<T> {
        Day { tasks: vec![] }
    }

    fn has_collisions(&self) -> bool {
        for (task1, task2) in self.tasks.iter().zip(self.tasks.iter().skip(1)) {
            if task1.span.collides(&task2.span) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Span;
    use crate::Time;

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
            Day::new(vec![Task::new(_span1, ()), Task::new(_span2, ()), Task::new(_span4, ()), Task::new(_span5, ())]).has_collisions(),
            false
        );
        assert_eq!(
            Day::new(vec![Task::new(_span1, ()), Task::new(_span2, ()), Task::new(_span6, ()), Task::new(_span5, ())]).has_collisions(),
            true
        );
    }
}
