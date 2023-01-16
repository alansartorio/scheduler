use serde::Deserialize;
use crate::{career_plan::CareerPlan, *};

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct BadListTest (    
    #[serde(deserialize_with = "from_bad_list")]
    Vec<i64>
);

#[test]
fn deserialize_bad_list() {
    let v: BadListTest = serde_json::from_str("null").unwrap();
    assert_eq!(v, BadListTest(vec![]));
    let v: BadListTest = serde_json::from_str(r#"{"list": 12}"#).unwrap();
    assert_eq!(v, BadListTest(vec![12]));
    let v: BadListTest = serde_json::from_str(r#"{"list": [14, 15]}"#).unwrap();
    assert_eq!(v, BadListTest(vec![14, 15]));
    let v: BadListTest = serde_json::from_str(r#"{"list": null}"#).unwrap();
    assert_eq!(v, BadListTest(vec![]));
}

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
        Code { high: 4, low: 10 }
    );

    assert_eq!(
        serde_json::from_str::<Code>("\"14.12\"").unwrap(),
        Code { high: 14, low: 12 }
    );
}

#[test]
fn serialize_code() {
    assert_eq!(
        serde_json::to_string(&Code { high: 0, low: 0 }).unwrap(),
        "\"00.00\"",
    );

    assert_eq!(
        serde_json::to_string(&Code { high: 4, low: 10 }).unwrap(),
        "\"04.10\"",
    );

    assert_eq!(
        serde_json::to_string(&Code { high: 14, low: 12 }).unwrap(),
        "\"14.12\"",
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
            subject_code: Code { high: 61, low: 82 },
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
    let parsed = serde_json::from_str::<SubjectCommissions>(include_str!("commissions.json"));

    dbg!(&parsed);
    assert!(parsed.is_ok());
}

#[test]
fn test_parse_student() {
    let parsed = serde_json::from_str::<Student>(include_str!("student.json"));

    dbg!(&parsed);
    assert!(parsed.is_ok());

    let parsed = parsed.unwrap();

    assert_eq!(parsed.code, "66666");
    assert_eq!(parsed.dni, "44444444");
    assert_eq!(parsed.career_code, "S");
    assert_eq!(
        parsed.active_career_plans,
        CareerPlans(vec![CareerPlanInfo {
            name: "S10 A - Rev18".to_owned(),
            career: "S".to_owned(),
            degree_level: DegreeLevel::Graduate,
            since: "2018-00-00T00:00:00-03:00".to_owned(),
        }])
    );
}

#[test]
fn test_parse_people() {
    let parsed = serde_json::from_str::<People>(include_str!("people.json"));

    dbg!(&parsed);
    assert!(parsed.is_ok());

    let parsed = parsed.unwrap();

    assert_eq!(parsed.dni, "44444444");
    assert_eq!(parsed.first_name, "NOMBRE");
    assert_eq!(parsed.last_name, "APELLIDO");
    assert_eq!(parsed.email_itba, "email@example.com");
    assert_eq!(parsed.links, vec![]);
}

#[test]
fn test_parse_simple_plan() {
    let parsed = serde_json::from_str::<CareerPlan>(
        r#"{
            "careerplan": {
                "name": "P10",
                "career": "P",
                "degreeLevel": "GRADUATE",
                "since": "2018-07-21T00:00:00-03:00",
                "section": [{
                    "name": "Section 1",
                    "terms": null,
                    "withoutTerm": null
                }]
            }
        }"#,
    );

    dbg!(&parsed);
    assert!(parsed.is_ok());
}

#[test]
fn test_parse_plan() {
    let parsed = serde_json::from_str::<CareerPlan>(include_str!("career-plan.json"));

    dbg!(&parsed);
    parsed.unwrap();
}
