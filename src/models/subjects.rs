use crate::models::{Collidable, Week};
pub use std::{error::Error, rc::Rc, str::FromStr, string::ParseError};
use std::{
    fmt::{Debug, Display},
    hash::Hash,
    sync::Weak,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Building {
    pub name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TaskInfo {
    pub subject: Weak<Subject>,
    pub building: Building,
}

#[derive(Debug, Clone)]
pub struct SubjectCommision {
    pub name: String,
    pub subject: Weak<Subject>,
    pub schedule: Week<TaskInfo>,
}
impl Display for SubjectCommision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.subject.upgrade().unwrap(), self.name)
    }
}
impl Collidable for SubjectCommision {
    fn collides(&self, other: &Self) -> bool {
        self.schedule.collides(&other.schedule)
    }
}
impl Hash for SubjectCommision {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.subject.upgrade().unwrap().code.hash(state);
    }
}
impl PartialEq for SubjectCommision {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
            && self
                .subject
                .upgrade()
                .unwrap()
                .eq(&other.subject.upgrade().unwrap())
    }
}
impl Eq for SubjectCommision {}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Code {
    pub high: u8,
    pub low: u8,
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

#[derive(Debug, Hash, PartialEq)]
pub struct Subject {
    pub code: Code,
    pub name: String,
    pub commissions: Vec<SubjectCommision>,
    pub credits: u8,
}

impl Eq for Subject {}
impl Display for Subject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {} ({})", self.code, self.name, self.credits)
    }
}
impl Subject {
    pub fn find_commission_by_id<'a>(&'a self, id: &str) -> Option<&'a SubjectCommision> {
        self.commissions.iter().find(|com| com.name == id)
    }
}

#[cfg(test)]
mod tests {
    use super::Code;

    #[test]
    fn code_to_string() {
        assert_eq!(Code { high: 3, low: 6 }.to_string(), "03.06");
        assert_eq!(Code { high: 10, low: 40 }.to_string(), "10.40");
    }
}
