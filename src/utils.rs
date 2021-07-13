use chrono::Utc;

const DEFAULT_CONFIG: base64::Config = base64::Config::new(base64::CharacterSet::Crypt, false);
pub(crate) fn encode(num: i64) -> String {
    base64::encode_config(num.to_string(), DEFAULT_CONFIG)
}
struct LastInfo {
    ts: i64,
    index: i64,
}
impl LastInfo {
    fn new() -> Self {
        Self { ts: 0, index: 0 }
    }
}
pub(crate) struct Yeast {
    last: LastInfo,
}

impl Yeast {
    pub(crate) fn new() -> Self {
        Yeast {
            last: LastInfo::new(),
        }
    }
}
impl Iterator for Yeast {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let now = Utc::now().timestamp_millis();
        if self.last.ts == now {
            self.last.index += 1;
            Some(format!("{}_{}", encode(now), encode(self.last.index - 1)))
        } else {
            self.last.ts = now;
            self.last.index = 0;
            Some(encode(now))
        }
    }
}
