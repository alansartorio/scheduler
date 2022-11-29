use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::{error::Error, str::FromStr, fmt::{Display, Debug}};
mod career_plan;
#[cfg(test)]
mod test;

pub use career_plan::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum SubjectType {
    #[serde(alias = "ANNUAL")]
    Annual,
    #[serde(alias = "NORMAL")]
    Normal,
    #[serde(alias = "SEMINARY")]
    Seminary,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct Code {
    pub high: u8,
    pub low: u8,
}

impl TryFrom<String> for Code {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (major, minor) = value
            .split_once('.')
            .ok_or_else::<String, _>(|| "Could not split at .".into())?;
        let major = major.parse()?;
        let minor = minor.parse()?;
        Ok(Self {
            high: major,
            low: minor,
        })
    }
}

impl From<Code> for String {
    fn from(code: Code) -> Self {
        format!("{:02}.{:02}", code.high, code.low)
    }
}

impl FromStr for Code {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (high, low) = s
            .split_once('.')
            .ok_or("Subject Code must contain a dot.")?;
        let high: u8 = high.parse()?;
        let low: u8 = low.parse()?;
        Ok(Code { high, low })
    }
}
impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}.{:02}", self.high, self.low)
    }
}
impl Debug for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(try_from = "String")]
pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: u8,
}

impl TryFrom<String> for Date {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let values: [u8; 3] = value
            .split('/')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|_| "Error parsing date.")?;
        let day = values[0];
        let month = values[1];
        let year = values[2];
        Ok(Self { day, month, year })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Day {
    #[serde(alias = "Monday", alias = "monday")]
    Monday,
    #[serde(alias = "Tuesday", alias = "tuesday")]
    Tuesday,
    #[serde(alias = "Wednesday", alias = "wednesday")]
    Wednesday,
    #[serde(alias = "Thursday", alias = "thursday")]
    Thursday,
    #[serde(alias = "Friday", alias = "friday")]
    Friday,
    #[serde(alias = "Saturday", alias = "saturday")]
    Saturday,
    #[serde(alias = "Sunday", alias = "sunday")]
    Sunday,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(try_from = "String")]
pub struct Time {
    pub hours: u8,
    pub minutes: u8,
}

impl TryFrom<String> for Time {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (hours, minutes) = value
            .split_once(':')
            .ok_or_else::<String, _>(|| "Could not split at :".into())?;
        let hours = hours.parse()?;
        let minutes = minutes.parse()?;
        Ok(Self { hours, minutes })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct Span {
    #[serde(rename = "hourFrom")]
    pub start: Time,
    #[serde(rename = "hourTo")]
    pub end: Time,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CommissionTime {
    pub day: Day,
    #[serde(rename = "classRoom")]
    pub classroom: Option<String>,
    pub building: String,
    #[serde(flatten)]
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
enum CommissionTimesEnum {
    Single(CommissionTime),
    Multiple(Vec<CommissionTime>),
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(from = "Option<CommissionTimesEnum>")]
pub struct CommissionTimes(pub Vec<CommissionTime>);

impl From<Option<CommissionTimesEnum>> for CommissionTimes {
    fn from(times: Option<CommissionTimesEnum>) -> Self {
        CommissionTimes(match times {
            Some(times) => match times {
                CommissionTimesEnum::Single(time) => vec![time],
                CommissionTimesEnum::Multiple(times) => times,
            },
            None => vec![],
        })
    }
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubjectCommission {
    pub subject_code: Code,
    pub subject_name: String,
    pub subject_type: SubjectType,
    pub course_start: Date,
    pub course_end: Date,
    pub commission_name: String,
    pub commission_id: String,
    #[serde_as(as = "DisplayFromStr")]
    pub quota: u64,
    #[serde_as(as = "DisplayFromStr")]
    pub enrolled_students: u64,
    pub course_commission_times: CommissionTimes,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InnerSubjectCommissions {
    course_commission: Vec<SubjectCommission>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OriginalSubjectCommissions {
    course_commissions: InnerSubjectCommissions,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(from = "OriginalSubjectCommissions")]
pub struct SubjectCommissions(pub Vec<SubjectCommission>);

impl From<OriginalSubjectCommissions> for SubjectCommissions {
    fn from(s: OriginalSubjectCommissions) -> Self {
        SubjectCommissions(s.course_commissions.course_commission)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum DegreeLevel {
    #[serde(alias = "GRADUATE")]
    Graduate,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CareerPlanInfo {
    pub name: String,
    pub career: String,
    pub degree_level: DegreeLevel,
    pub since: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
enum ActiveCareerPlansEnum {
    Single(CareerPlanInfo),
    Multiple(Vec<CareerPlanInfo>),
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", from = "Option<ActiveCareerPlansEnum>")]
struct InnerCareerPlans {
    career_plans: Vec<CareerPlanInfo>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OriginalCareerPlans {
    career_plans: InnerCareerPlans,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(from = "OriginalCareerPlans")]
pub struct CareerPlans(pub Vec<CareerPlanInfo>);

impl From<Option<ActiveCareerPlansEnum>> for InnerCareerPlans {
    fn from(career_plans: Option<ActiveCareerPlansEnum>) -> Self {
        InnerCareerPlans {
            career_plans: match career_plans {
                Some(career_plans) => match career_plans {
                    ActiveCareerPlansEnum::Single(career_plan) => vec![career_plan],
                    ActiveCareerPlansEnum::Multiple(career_plans) => career_plans,
                },
                None => vec![],
            },
        }
    }
}

impl From<OriginalCareerPlans> for CareerPlans {
    fn from(og: OriginalCareerPlans) -> Self {
        Self(og.career_plans.career_plans)
    }
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub rel: String,
    pub href: String,
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Student {
    pub code: String,
    pub dni: String,
    pub civic_id: String,
    pub career: String,
    pub career_code: String,
    pub active_career_plans: CareerPlans,
    pub plan: String,
    pub email: String,
    pub start_year: String,
    pub start_period: String,
    pub student_type: String,
    pub links: Vec<Link>,
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct People {
    pub dni: String,
    pub first_name: String,
    pub last_name: String,
    pub email_itba: String,
    pub links: Vec<Link>,
}
