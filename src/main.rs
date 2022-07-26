use std::collections::HashSet;
use std::fmt::Debug;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use itertools::Itertools;
use clap::Parser;
use scheduler::loader::{load, Code, SubjectCommision};
use scheduler::option_generator::generate;
mod subject_iter;
use subject_iter::SubjectIterable;

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
