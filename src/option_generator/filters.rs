use std::ops::RangeBounds;

use crate::models::SubjectCommision;

type Choice = Vec<Option<SubjectCommision>>;

pub trait ChoiceFilter {
    fn filter(&self, item: &Choice) -> bool;
}

pub struct ChoiceFilterIterator<I: Iterator<Item = Choice>, F: ChoiceFilter> {
    iterator: I,
    filter: F,
}

impl<I: Iterator<Item = Choice>, F: ChoiceFilter> Iterator for ChoiceFilterIterator<I, F> {
    type Item = Choice;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.find(|c| self.filter.filter(c))
    }
}

pub trait ChoiceIterator: Iterator<Item = Choice> {
    fn filter_choices<F: ChoiceFilter>(self, filter: F) -> ChoiceFilterIterator<Self, F>
    where
        Self: Sized,
    {
        ChoiceFilterIterator {
            iterator: self,
            filter,
        }
    }
}

impl<I: Iterator<Item = Choice>> ChoiceIterator for I {}

pub struct CreditCount<R: RangeBounds<u32>> {
    valid_range: R,
}

impl<R: RangeBounds<u32>> CreditCount<R> {
    pub fn new(valid_range: R) -> Self {
        Self { valid_range }
    }
}

impl<R: RangeBounds<u32>> ChoiceFilter for CreditCount<R> {
    fn filter(&self, item: &Choice) -> bool {
        let credits = item
            .iter()
            .flatten()
            .map(|c| c.subject.upgrade().unwrap().borrow().credits as u32)
            .sum();
        self.valid_range.contains(&credits)
    }
}

pub struct SubjectCount<R: RangeBounds<u32>> {
    valid_range: R,
}

impl<R: RangeBounds<u32>> SubjectCount<R> {
    pub fn new(valid_range: R) -> Self {
        Self { valid_range }
    }
}

impl<R: RangeBounds<u32>> ChoiceFilter for SubjectCount<R> {
    fn filter(&self, item: &Choice) -> bool {
        let credits = item.iter().flatten().count() as u32;
        self.valid_range.contains(&credits)
    }
}
