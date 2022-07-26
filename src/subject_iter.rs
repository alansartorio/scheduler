use scheduler::models::{Code, Subject};
use std::{collections::HashSet, rc::Rc};

pub struct Whitelist<'a, I: Iterator<Item = Rc<Subject>>> {
    iter: I,
    list: &'a HashSet<Code>,
}
impl<'a, I: Iterator<Item = Rc<Subject>>> Whitelist<'a, I> {
    pub fn new(iter: I, list: &'a HashSet<Code>) -> Self {
        Self { iter, list }
    }
}
impl<'a, I: Iterator<Item = Rc<Subject>>> Iterator for Whitelist<'a, I> {
    type Item = Rc<Subject>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find(|i| self.list.contains(&i.code))
    }
}

pub struct Blacklist<'a, I: Iterator<Item = Rc<Subject>>> {
    iter: I,
    list: &'a HashSet<Code>,
}
impl<'a, I: Iterator<Item = Rc<Subject>>> Blacklist<'a, I> {
    pub fn new(iter: I, list: &'a HashSet<Code>) -> Self {
        Self { iter, list }
    }
}
impl<'a, I: Iterator<Item = Rc<Subject>>> Iterator for Blacklist<'a, I> {
    type Item = Rc<Subject>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find(|i| !self.list.contains(&i.code))
    }
}

pub trait SubjectIterable: Iterator<Item = Rc<Subject>> {
    fn get_by_code(&mut self, code: Code) -> Option<Rc<Subject>>
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

impl<T: Iterator<Item = Rc<Subject>>> SubjectIterable for T {}
