use std::collections::HashMap;

use crate::{Code, DegreeLevel};
use serde::{de::Error, Deserialize, Deserializer};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum BadList<T> {
    Multiple(Vec<T>),
    Single(T),
    None,
}

impl<T> From<BadList<T>> for Vec<T> {
    fn from(bad: BadList<T>) -> Self {
        match bad {
            BadList::Multiple(l) => l,
            BadList::Single(e) => vec![e],
            BadList::None => vec![],
        }
    }
}

pub fn from_bad_list<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let l: Option<HashMap<&str, BadList<T>>> = Deserialize::deserialize(deserializer)?;

    match l {
        Some(e) => e
            .into_values()
            .next()
            .map(Into::into)
            .ok_or_else(|| D::Error::custom("Map did not contain any key.")),
        None => Ok(vec![]),
    }
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Term {
    #[serde_as(as = "DisplayFromStr")]
    pub year: u8,
    #[serde_as(as = "DisplayFromStr")]
    pub period: u8,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
enum DependenciesEnum {
    Single(Code),
    Multiple(Vec<Code>),
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", from = "DependenciesEnum")]
struct InnerDependencies {
    dependencies: Vec<Code>,
}

impl From<DependenciesEnum> for InnerDependencies {
    fn from(dependencies: DependenciesEnum) -> Self {
        InnerDependencies {
            dependencies: match dependencies {
                DependenciesEnum::Single(dependency) => vec![dependency],
                DependenciesEnum::Multiple(dependencies) => dependencies,
            },
        }
    }
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubjectEntry {
    pub name: String,
    pub code: Code,
    #[serde_as(as = "DisplayFromStr")]
    pub credits: u8,
    #[serde(deserialize_with = "from_bad_list")]
    pub dependencies: Vec<Code>,
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SectionEntry {
    pub name: String,
    #[serde_as(as = "DisplayFromStr")]
    pub credits: u8,
    #[serde(deserialize_with = "from_bad_list")]
    pub dependencies: Vec<Code>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Entry {
    Subject(SubjectEntry),
    Section(SectionEntry),
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TermWithEntries {
    #[serde(flatten)]
    pub term: Term,
    #[serde(deserialize_with = "from_bad_list")]
    pub entries: Vec<Entry>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Section {
    pub name: String,
    #[serde(deserialize_with = "from_bad_list")]
    pub terms: Vec<TermWithEntries>,
    #[serde(deserialize_with = "from_bad_list")]
    pub without_term: Vec<Entry>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InnerCareerPlan {
    name: String,
    career: String,
    degree_level: DegreeLevel,
    since: String,
    section: Vec<Section>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct OriginalCareerPlan {
    #[serde(rename = "careerplan")]
    career_plan: InnerCareerPlan,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(from = "OriginalCareerPlan")]
pub struct CareerPlan {
    pub name: String,
    pub career: String,
    pub degree_level: DegreeLevel,
    pub since: String,
    #[serde(deserialize_with = "from_bad_list")]
    pub sections: Vec<Section>,
}

impl From<OriginalCareerPlan> for CareerPlan {
    fn from(og: OriginalCareerPlan) -> Self {
        CareerPlan {
            name: og.career_plan.name,
            career: og.career_plan.career,
            degree_level: og.career_plan.degree_level,
            since: og.career_plan.since,
            sections: og.career_plan.section,
        }
    }
}
