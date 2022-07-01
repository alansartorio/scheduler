#![feature(generators)]
#![feature(generic_arg_infer)]

mod loader;
mod models;
mod option_generator;

use std::collections::HashSet;
use std::fs::read_to_string;
use std::rc::Rc;

use itertools::Itertools;
use loader::loader::{Code, Subject};
//use sqlite;

use extend::ext;

use crate::loader::loader::{load, SubjectCommision};
use crate::option_generator::Group;

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
    let code2 = load_codes!("../data/available-codes.txt");
    //let codes = code1.intersection(&code2).cloned().collect::<HashSet<_>>();
    let codes = code2;
    let codes = codes
        .difference(&blacklisted)
        .cloned()
        .collect::<HashSet<_>>();

    let subjects = load().unwrap();
    let subjects = subjects
        .iter()
        .cloned()
        .whitelist_codes(codes)
        .iter()
        .map(|sub| Group {
            items: sub.commissions.clone(),
            mandatory: mandatory.contains(&sub.code),
        })
        .collect_vec();
    dbg!(&subjects);

    let options = option_generator::generate::<SubjectCommision>(&subjects);

    for option in options {
        let subject_count = option.iter().filter_map(|&a| a).count();
        if subject_count < 4 {
            continue;
        }
        let filtered = option.iter().filter_map(|&a| a).collect_vec();

        println!(
            "{}",
            filtered
                .iter()
                .enumerate()
                .map(|(_i, com)| com.to_string())
                .join(", ") //.join(&" \u{2588} ".green().to_string())
        );
        //dbg!(combined);
    }

    //dbg!(Week::combine(&cloud_a.schedule, &eco_km.schedule));

    //println!("{:?}", combined);
}
