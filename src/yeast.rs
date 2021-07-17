use crate::encode::{decode, encode};
use chrono::Utc;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

/// A unique id, based on timestamp.
#[derive(Clone, Debug)]
pub struct Yeast {
    ts: i64,
    index: Option<i64>,
}
impl Yeast {
    fn empty() -> Self {
        Self { ts: 0, index: None }
    }
    fn new(ts: i64, index: Option<i64>) -> Self {
        Self { ts, index }
    }
    fn update(&mut self, now: i64) {
        if self.ts == now {
            *self.index.get_or_insert(-1) += 1;
        } else {
            self.ts = now;
            self.index = None;
        }
    }
    /// get this id timestamp.
    pub fn timestamp_millis(&self) -> i64 {
        self.ts
    }
}
/// Url Safe representation of the id
impl Display for Yeast {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.index {
            None => write!(f, "{}", encode(self.ts)),
            Some(index) => write!(f, "{}.{}", encode(self.ts), encode(index)),
        }
    }
}

/// Convert a String back
impl TryFrom<String> for Yeast {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.split_once(".") {
            None => Ok(Yeast::new(decode(value.as_str())?, None)),
            Some(data) => Ok(Yeast::new(decode(data.0)?, Some(decode(data.1)?))),
        }
    }
}
pub(crate) struct YeastGenerator {
    last_yeast: Yeast,
}

impl YeastGenerator {
    pub(crate) fn new() -> Self {
        YeastGenerator {
            last_yeast: Yeast::empty(),
        }
    }
}
impl Iterator for YeastGenerator {
    type Item = Yeast;

    fn next(&mut self) -> Option<Self::Item> {
        self.last_yeast.update(Utc::now().timestamp_millis());
        Some(self.last_yeast.clone())
    }
}
