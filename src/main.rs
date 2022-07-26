#![feature(generators)]
#![feature(generic_arg_infer)]

use std::collections::HashSet;
use std::fmt::Debug;
use std::fs::read_to_string;
use std::path::{PathBuf, Path};
use std::rc::Rc;

use itertools::Itertools;
//use sqlite;

use scheduler::loader::{load, Code, Subject, SubjectCommision};
use scheduler::option_generator::generate;

struct Whitelist<'a, I: Iterator<Item = Rc<Subject>>> {
    iter: I,
    list: &'a HashSet<Code>,
}
impl<'a, I: Iterator<Item = Rc<Subject>>> Whitelist<'a, I> {
    fn new(iter: I, list: &'a HashSet<Code>) -> Self {
        Self { iter, list }
    }
}
impl<'a, I: Iterator<Item = Rc<Subject>>> Iterator for Whitelist<'a, I> {
    type Item = Rc<Subject>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find(|i| self.list.contains(&i.code))
    }
}

struct Blacklist<'a, I: Iterator<Item = Rc<Subject>>> {
    iter: I,
    list: &'a HashSet<Code>,
}
impl<'a, I: Iterator<Item = Rc<Subject>>> Blacklist<'a, I> {
    fn new(iter: I, list: &'a HashSet<Code>) -> Self {
        Self { iter, list }
    }
}
impl<'a, I: Iterator<Item = Rc<Subject>>> Iterator for Blacklist<'a, I> {
    type Item = Rc<Subject>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find(|i| !self.list.contains(&i.code))
    }
}

trait IterExtensions: Iterator<Item = Rc<Subject>> {
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

fn load_codes(path: &Path) -> HashSet<Code> {
    HashSet::<_>::from_iter(
        read_to_string(path)
            .unwrap()
            .lines()
            .map(str::trim)
            .filter(|line| !line.starts_with('#'))
            .filter(|line| !line.is_empty())
            .map(|line| &line[..5])
            .map(str::parse::<Code>)
            .map(Result::unwrap),
    )
}

fn main() {
    let args = Args::parse();
    let mut files = args.files.into_iter();
    let mut codes = load_codes(&files.next().unwrap());
    for file in files {
        codes = codes.intersection(&load_codes(&file)).cloned().collect();
    }
    dbg!(&codes);
    let mandatory = load_codes(&args.mandatory);
    dbg!(&mandatory);

    if let Some(blacklisted) = args.blacklisted {
        let blacklisted = load_codes(&blacklisted);

        codes = codes.difference(&blacklisted).cloned().collect();
    }

    let subjects = load().unwrap();
    let optional_subjects = subjects
        .iter()
        .cloned()
        .whitelist_codes(&codes)
        .blacklist_codes(&mandatory)
        .map(|sub| sub.commissions.clone())
        .collect_vec();

    let mandatory_subjects = subjects
        .iter()
        .cloned()
        .whitelist_codes(&mandatory)
        .map(|sub| sub.commissions.clone())
        .collect_vec();

    let options = generate::<SubjectCommision>(&mandatory_subjects, &optional_subjects);

    for option in options {
        let subject_count = option.iter().filter_map(|&a| a).count();
        if !(4..=5).contains(&subject_count) {
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
