#![feature(generators)]
#![feature(generic_arg_infer)]

use std::collections::HashSet;
use std::fmt::Debug;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::rc::Rc;

use itertools::Itertools;
//use sqlite;

use scheduler::loader::{load, Code, Subject, SubjectCommision};
use scheduler::option_generator::generate;

fn whitelist_codes<I: Iterator<Item = Rc<Subject>>>(
    codes: I,
    list: HashSet<Code>,
) -> impl Iterator<Item = Rc<Subject>> {
    codes
        .filter(move |sub| list.contains(&sub.code))
        .map(|s| s.clone())
}

fn blacklist_codes<I: Iterator<Item = Rc<Subject>>>(
    codes: I,
    list: HashSet<Code>,
) -> impl Iterator<Item = Rc<Subject>> {
    codes
        .filter(move |sub| !list.contains(&sub.code))
        .map(|s| s.clone())
}

//struct Whitelist<I: Iterator<Item = Rc<Subject>>> {
    //iter: I,
    //list: HashSet<Code>,
//}

//impl<I: Iterator<Item = Rc<Subject>>> Iterator for Whitelist<I> {
    //type Item = Rc<Subject>;

    //fn next(&mut self) -> Option<Self::Item> {
        //match self.iter.next() {
            //Some(i) if self.list.contains(&i.code) => Some(i),
            //_ => None,
        //}
    //}
//}

trait IterExtensions: Iterator<Item = Rc<Subject>> {
    fn get_by_code(&mut self, code: Code) -> Option<Rc<Subject>>
    where
        Self: Sized,
    {
        self.find(|sub| (*sub).code == code).map(|s| s.clone())
    }

    fn whitelist_codes(
        self: Box<Self>,
        codes: HashSet<Code>,
    ) -> Box<dyn Iterator<Item = Rc<Subject>>> {
        Box::new(whitelist_codes(self, codes))
    }

    fn blacklist_codes(
        self: Box<Self>,
        codes: HashSet<Code>,
    ) -> Box<dyn Iterator<Item = Rc<Subject>>> {
        Box::new(blacklist_codes(self, codes))
    }
}

impl<T: Iterator<Item = Rc<Subject>>> IterExtensions for T {}

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    files: Vec<PathBuf>,
    #[clap(short, long, value_parser)]
    mandatory: PathBuf,
    #[clap(short, long, value_parser)]
    blacklisted: Option<PathBuf>,
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
    let args = Args::parse();
    let mut files = args.files.into_iter();
    let mut codes = load_codes!(files.next().unwrap());
    for file in files {
        codes = codes.intersection(&load_codes!(file)).cloned().collect();
        dbg!(&codes);
    }
    let mandatory = load_codes!(args.mandatory);

    if let Some(blacklisted) = args.blacklisted {
        let blacklisted = load_codes!(blacklisted);

        codes = codes.difference(&blacklisted).cloned().collect();
    }

    let subjects = load().unwrap();
    let optional_subjects = Box::new(subjects.iter().cloned())
        .whitelist_codes(codes)
        .blacklist_codes(mandatory.clone())
        .map(|sub| sub.commissions.clone())
        .collect_vec();

    let mandatory_subjects = Box::new(subjects.into_iter())
        .whitelist_codes(mandatory.clone())
        .map(|sub| sub.commissions.clone())
        .collect_vec();

    let options = generate::<SubjectCommision>(&mandatory_subjects, &optional_subjects);

    for option in options {
        let subject_count = option.iter().filter_map(|&a| a).count();
        if subject_count < 4 || subject_count > 5 {
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
