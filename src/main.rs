#![feature(generators)]
#![feature(generic_arg_infer)]

mod loader;
mod models;
mod option_generator;

use std::collections::HashSet;
use std::iter::Filter;

use loader::loader::{Code, Subject};
//use sqlite;
use models::{span::Span, time::Time};

use extend::ext;

use crate::loader::loader::{load, SubjectCommision};
use crate::models::combinable::Combinable;
use crate::models::week::Week;

#[ext]
impl<I: Iterator<Item = &'a Subject>, 'a> I {
    fn get_by_code(&mut self, code: Code) -> Option<&'a Subject> {
        self.find(|sub| sub.code == code)
    }

    fn whitelist_codes(self, codes: HashSet<Code>) -> Vec<&'a Subject> {
        self.filter(|sub| codes.contains(&sub.code)).collect()
    }
}

fn main() {
    let span = Span::new(Time::new(3, 4), "3:05".parse().unwrap());

    println!("{}", span);

    let subjects = load().unwrap();
    let subjects = subjects.iter().whitelist_codes(HashSet::from(
        ["82.08", "61.23"].map(str::parse).map(Result::unwrap),
    ));

    dbg!(subjects);

    //dbg!(Week::combine(&cloud_a.schedule, &eco_km.schedule));

    //println!("{:?}", combined);
}
