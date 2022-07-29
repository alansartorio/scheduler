use clap::Parser;
use itertools::Itertools;
use scheduler::loaders::json_loader::load;
use scheduler::models::Code;
use scheduler::option_generator::filters::{ChoiceIterator, CreditCount, SubjectCount};
use scheduler::option_generator::generate;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
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

    let subjects = load(Path::new("json_parser/src/test.json")).unwrap();
    let optional_subjects = subjects
        .iter()
        .cloned()
        .whitelist_codes(&codes)
        .blacklist_codes(&mandatory)
        .map(|sub| (sub.code.clone(), sub.commissions.clone()))
        .collect_vec();

    let mandatory_subjects = subjects
        .iter()
        .cloned()
        .whitelist_codes(&mandatory)
        .map(|sub| (sub.code.clone(), sub.commissions.clone()))
        .collect_vec();

    let options = generate(mandatory_subjects, optional_subjects, HashSet::new())
        .filter_choices(SubjectCount::new(4..=5))
        .filter_choices(CreditCount::new(20..=30));

    for option in options {
        let filtered = option.into_iter().flatten().collect_vec();

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
