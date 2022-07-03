#![feature(generators)]
#![feature(generic_arg_infer)]

use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::rc::Rc;

use itertools::Itertools;
use scheduler::loader::loader::{Code, Subject};
//use sqlite;

use extend::ext;

use scheduler::loader::loader::{load, SubjectCommision};
use scheduler::option_generator::{generate, Group};

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

    let options = generate::<SubjectCommision>(&subjects);

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
