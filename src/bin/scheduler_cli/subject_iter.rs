use scheduler::models::{Code, Subject};
use std::{collections::HashSet, sync::Arc};

pub struct Whitelist<'a, I: Iterator<Item = Arc<Subject>>> {
    iter: I,
    list: &'a HashSet<Code>,
}
impl<'a, I: Iterator<Item = Arc<Subject>>> Whitelist<'a, I> {
    pub fn new(iter: I, list: &'a HashSet<Code>) -> Self {
        Self { iter, list }
    }
}
impl<'a, I: Iterator<Item = Arc<Subject>>> Iterator for Whitelist<'a, I> {
    type Item = Arc<Subject>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find(|i| self.list.contains(&i.code))
    }
}

pub struct Blacklist<'a, I: Iterator<Item = Arc<Subject>>> {
    iter: I,
    list: &'a HashSet<Code>,
}
impl<'a, I: Iterator<Item = Arc<Subject>>> Blacklist<'a, I> {
    pub fn new(iter: I, list: &'a HashSet<Code>) -> Self {
        Self { iter, list }
    }
}
impl<'a, I: Iterator<Item = Arc<Subject>>> Iterator for Blacklist<'a, I> {
    type Item = Arc<Subject>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find(|i| !self.list.contains(&i.code))
    }
}

pub trait SubjectIterable: Iterator<Item = Arc<Subject>> {
    fn get_by_code(&mut self, code: Code) -> Option<Arc<Subject>>
    where
        Self: Sized,
    {
        self.find(|sub| (*sub).code == code)
    }

    fn whitelist_codes(self, codes: &HashSet<Code>) -> Whitelist<Self>
    where
        Self: Sized,
    {
        Whitelist::new(self, codes)
    }

    fn blacklist_codes(self, codes: &HashSet<Code>) -> Blacklist<Self>
    where
        Self: Sized,
    {
        Blacklist::new(self, codes)
    }
}

impl<T: Iterator<Item = Arc<Subject>>> SubjectIterable for T {}
