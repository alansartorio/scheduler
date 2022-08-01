use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum SubjectType {
    #[serde(alias = "ANNUAL")]
    Annual,
    #[serde(alias = "NORMAL")]
    Normal,
    #[serde(alias = "SEMINARY")]
    Seminary,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(try_from = "String")]
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
        Ok(Self { high: major, low: minor })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Span {
    #[serde(rename = "hourFrom")]
    pub start: Time,
    #[serde(rename = "hourTo")]
    pub end: Time,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CommissionTime {
    pub day: Day,
    #[serde(rename = "classRoom")]
    pub classroom: Option<String>,
    pub building: String,
    #[serde(flatten)]
    pub span: Span,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
enum CommissionTimesEnum {
    Single(CommissionTime),
    Multiple(Vec<CommissionTime>),
}

#[derive(Debug, PartialEq, Deserialize)]
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
#[derive(Debug, PartialEq, Deserialize)]
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

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InnerSubjectCommissions {
    course_commission: Vec<SubjectCommission>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OriginalSubjectCommissions {
    course_commissions: InnerSubjectCommissions,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(from = "OriginalSubjectCommissions")]
pub struct SubjectCommissions(pub Vec<SubjectCommission>);

impl From<OriginalSubjectCommissions> for SubjectCommissions {
    fn from(s: OriginalSubjectCommissions) -> Self {
        SubjectCommissions(s.course_commissions.course_commission)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_day() {
        assert_eq!(
            serde_json::from_str::<Day>("\"MONDAY\"").unwrap(),
            Day::Monday
        );
        assert_eq!(
            serde_json::from_str::<Day>("\"monday\"").unwrap(),
            Day::Monday
        );
        assert_eq!(
            serde_json::from_str::<Day>("\"Monday\"").unwrap(),
            Day::Monday
        );
    }

    #[test]
    fn deserialize_time() {
        assert_eq!(
            serde_json::from_str::<Time>("\"00:00\"").unwrap(),
            Time {
                hours: 0,
                minutes: 0,
            }
        );

        assert_eq!(
            serde_json::from_str::<Time>("\"04:10\"").unwrap(),
            Time {
                hours: 4,
                minutes: 10,
            }
        );

        assert_eq!(
            serde_json::from_str::<Time>("\"14:12\"").unwrap(),
            Time {
                hours: 14,
                minutes: 12,
            }
        );
    }

    #[test]
    fn deserialize_code() {
        assert_eq!(
            serde_json::from_str::<Code>("\"00.00\"").unwrap(),
            Code { high: 0, low: 0 }
        );

        assert_eq!(
            serde_json::from_str::<Code>("\"04.10\"").unwrap(),
            Code {
                high: 4,
                low: 10,
            }
        );

        assert_eq!(
            serde_json::from_str::<Code>("\"14.12\"").unwrap(),
            Code {
                high: 14,
                low: 12,
            }
        );
    }

    #[test]
    fn deserialize_date() {
        assert_eq!(
            serde_json::from_str::<Date>("\"10/7/22\"").unwrap(),
            Date {
                day: 10,
                month: 7,
                year: 22,
            }
        );

        assert_eq!(
            serde_json::from_str::<Date>("\"7/9/12\"").unwrap(),
            Date {
                day: 7,
                month: 9,
                year: 12,
            }
        );
    }

    #[test]
    fn deserialize_times() {
        assert_eq!(
            serde_json::from_str::<CommissionTimes>(
                r#"{
                "day": "MONDAY",
                "classRoom": "Virtual",
                "building": "External",
                "hourFrom": "18:30",
                "hourTo": "21:30"
            }"#
            )
            .unwrap(),
            CommissionTimes(vec![CommissionTime {
                day: Day::Monday,
                classroom: Some("Virtual".to_string()),
                building: "External".to_string(),
                span: Span {
                    start: Time {
                        hours: 18,
                        minutes: 30
                    },
                    end: Time {
                        hours: 21,
                        minutes: 30
                    }
                }
            }])
        );

        assert_eq!(
            serde_json::from_str::<CommissionTimes>(r#"null"#).unwrap(),
            CommissionTimes(vec![])
        );

        assert_eq!(
            serde_json::from_str::<CommissionTimes>(
                r#"[
                    {
                        "day": "MONDAY",
                        "classRoom": "Virtual",
                        "building": "External",
                        "hourFrom": "18:30",
                        "hourTo": "21:30"
                    },
                    {
                        "day": "WEDNESDAY",
                        "classRoom": "Virtual",
                        "building": "External",
                        "hourFrom": "18:30",
                        "hourTo": "21:30"
                    }
                ]"#
            )
            .unwrap(),
            CommissionTimes(vec![
                CommissionTime {
                    day: Day::Monday,
                    classroom: Some("Virtual".to_string()),
                    building: "External".to_string(),
                    span: Span {
                        start: Time {
                            hours: 18,
                            minutes: 30
                        },
                        end: Time {
                            hours: 21,
                            minutes: 30
                        }
                    }
                },
                CommissionTime {
                    day: Day::Wednesday,
                    classroom: Some("Virtual".to_string()),
                    building: "External".to_string(),
                    span: Span {
                        start: Time {
                            hours: 18,
                            minutes: 30
                        },
                        end: Time {
                            hours: 21,
                            minutes: 30
                        }
                    }
                }
            ])
        );
    }

    #[test]
    fn deserialize_full() {
        assert_eq!(
            serde_json::from_str::<SubjectCommission>(
                r#"{
            "subjectCode": "61.82",
            "subjectName": "Macroeconomía",
            "subjectType": "NORMAL",
            "courseStart": "23/07/22",
            "courseEnd": "31/12/22",
            "commissionName": "B",
            "commissionId": "34709",
            "quota": "0",
            "enrolledStudents": "7",
            "courseCommissionTimes": {
              "day": "TUESDAY",
              "classRoom": "Presencial",
              "building": "External",
              "hourFrom": "14:00",
              "hourTo": "17:00"
            }
          }"#
            )
            .unwrap(),
            SubjectCommission {
                subject_code: Code {
                    high: 61,
                    low: 82
                },
                subject_name: "Macroeconomía".to_string(),
                subject_type: SubjectType::Normal,
                course_start: Date {
                    day: 23,
                    month: 7,
                    year: 22
                },
                course_end: Date {
                    day: 31,
                    month: 12,
                    year: 22
                },
                commission_name: "B".to_string(),
                commission_id: "34709".to_string(),
                quota: 0,
                enrolled_students: 7,
                course_commission_times: CommissionTimes(vec![CommissionTime {
                    day: Day::Tuesday,
                    classroom: Some("Presencial".to_string()),
                    building: "External".to_string(),
                    span: Span {
                        start: Time {
                            hours: 14,
                            minutes: 0,
                        },
                        end: Time {
                            hours: 17,
                            minutes: 0,
                        }
                    }
                }])
            }
        )
    }

    #[test]
    fn test_file() {
        let parsed = serde_json::from_str::<SubjectCommissions>(include_str!("test.json"));

        dbg!(&parsed);
        assert!(parsed.is_ok());
    }
}
