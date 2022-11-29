use crate::{Code, DegreeLevel};
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

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

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OriginalDependencies {
    dependency: InnerDependencies,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", from = "Option<OriginalDependencies>")]
pub struct Dependencies(pub Vec<Code>);

impl From<Option<OriginalDependencies>> for Dependencies {
    fn from(og: Option<OriginalDependencies>) -> Self {
        Dependencies(match og {
            Some(og) => og.dependency.dependencies,
            None => vec![],
        })
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
    pub dependencies: Dependencies,
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SectionEntry {
    pub name: String,
    #[serde_as(as = "DisplayFromStr")]
    pub credits: u8,
    pub dependencies: Dependencies,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Entry {
    Subject(SubjectEntry),
    Section(SectionEntry),
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entries {
    pub entry: Vec<Entry>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TermWithEntries {
    #[serde(flatten)]
    pub term: Term,
    pub entries: Entries,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithTerm {
    pub term: Vec<TermWithEntries>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithoutTerm {
    pub without_term: Vec<Entry>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Section {
    pub name: String,
    pub terms: Option<WithTerm>,
    pub without_term: Option<WithoutTerm>,
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
