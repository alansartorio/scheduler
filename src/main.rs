#![feature(generators)]
#![feature(generic_arg_infer)]

mod loader;
mod models;
mod option_generator;

use std::borrow::Borrow;
use std::collections::HashSet;
use std::rc::Rc;

use itertools::Itertools;
use loader::loader::{Code, Subject};
//use sqlite;

use extend::ext;

use crate::loader::loader::{load, SubjectCommision};
use crate::models::combinable::Combinable;
use crate::models::week::Week;

#[ext]
impl<I: Iterator<Item = Rc<Subject>>> I {
    fn get_by_code(&mut self, code: Code) -> Option<Rc<Subject>> {
        self.find(|sub| (*sub).code == code).map(|s| s.clone())
    }

    fn whitelist_codes(self, codes: HashSet<Code>) -> Vec<Rc<Subject>> {
        self.filter(|sub| codes.contains(&sub.code)).map(|s| s.clone()).collect_vec()
    }
}

fn main() {
    macro_rules! load_codes {
        ($file: expr) => {
            HashSet::<_>::from_iter(
                include_str!($file)
                    .lines()
                    .map(str::parse::<Code>)
                    .map(Result::unwrap),
            )
        };
    }
    let code1 = load_codes!("../../data/available-codes.txt");
    dbg!(&code1);
    let code2 = load_codes!("../../data/available-codes2.txt");
    dbg!(&code2);
    let codes = code1.intersection(&code2);

    dbg!(codes.clone().collect_vec());

    let subjects = load().unwrap();
    let subjects = subjects
        .iter()
        .cloned()
        .whitelist_codes(HashSet::from_iter(codes.map(Clone::clone)))
        .iter()
        .map(|sub| sub.commissions.clone())
        .collect_vec();

    let options = option_generator::generate::<SubjectCommision>(&subjects);

    for option in options {
        if option.iter().filter_map(|&a| a).count() < 4 {
            continue;
        }
        let filtered = option.iter().filter_map(|&a| a).collect_vec();
        let _combined = filtered.iter().map(|c| &c.schedule).fold(Week::empty(), |a, b| Week::combine(&a, &b));
        println!("{}", filtered.iter().fold(String::new(), |acc, &com| acc + &com.to_string() + " | "));
        //dbg!(combined);
    }

    //dbg!(Week::combine(&cloud_a.schedule, &eco_km.schedule));

    //println!("{:?}", combined);
}
