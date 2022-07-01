#![feature(generators)]
#![feature(generic_arg_infer)]

mod loader;
mod models;
mod option_generator;

use std::cell::RefCell;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::rc::Rc;

use colored::Colorize;
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

    fn blacklist_codes(self, codes: HashSet<Code>) -> Vec<Rc<Subject>> {
        self.filter(|sub| !codes.contains(&sub.code))
            .map(|s| s.clone())
            .collect_vec()
    }

    fn whitelist_codes(self, codes: HashSet<Code>) -> Vec<Rc<Subject>> {
        self.filter(|sub| codes.contains(&sub.code))
            .map(|s| s.clone())
            .collect_vec()
    }
}

fn main() {
    macro_rules! load_codes {
        ($file: expr) => {
            HashSet::<_>::from_iter(
                read_to_string($file)
                    .unwrap()
                    .lines()
                    .map(str::trim)
                    .filter(|line| !line.starts_with("#"))
                    .filter(|line| !line.is_empty())
                    .map(|line| &line[..5])
                    .map(str::parse::<Code>)
                    .map(Result::unwrap),
            )
        };
    }
    let mandatory = load_codes!("../data/mandatory.txt");
    let blacklisted = load_codes!("../data/blacklisted.txt");
    //let code1 = load_codes!("../data/available-codes.txt");
    let code2 = load_codes!("../data/available-aldo.txt");
    //let codes = code1.intersection(&code2).cloned().collect::<HashSet<_>>();
    let codes = code2;
    let codes = codes
        .difference(&blacklisted)
        .cloned()
        .collect::<HashSet<_>>();

    dbg!(codes.clone());

    let subjects = load().unwrap();
    let subjects = subjects
        .iter()
        .cloned()
        .whitelist_codes(codes)
        .iter()
        .map(|sub| sub.commissions.clone())
        .collect_vec();

    let options = option_generator::generate::<SubjectCommision>(&subjects);

    for option in options {
        let subject_count = option.iter().filter_map(|&a| a).count();
        if subject_count < 6 || subject_count > 8 {
            continue;
        }
        let filtered = option.iter().filter_map(|&a| a).collect_vec();
        if !mandatory.iter().all(|m| {
            filtered
                .iter()
                .map(|com| RefCell::borrow(&com.subject).upgrade().unwrap().code)
                .contains(m)
        }) {
            continue;
        }
        //let combined = filtered
            //.iter()
            //.map(|c| &c.schedule)
            //.fold(Week::empty(), |a, b| Week::combine(&a, &b));
        //if combined
            //.days
            //.iter()
            //.any(|(_day, day_data)| day_data.has_collisions)
        //{
            //panic!("This should not have a collision.");
        //}

        println!(
            "{}",
            filtered
                .iter()
                .enumerate()
                .map(|(_i, com)| com.to_string())
                .join(", ")
                //.join(&" \u{2588} ".green().to_string())
        );
        //dbg!(combined);
    }

    //dbg!(Week::combine(&cloud_a.schedule, &eco_km.schedule));

    //println!("{:?}", combined);
}
