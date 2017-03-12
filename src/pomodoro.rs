use chrono::{UTC, datetime};
use uuid::Uuid;
use toml;

pub type PomTime = datetime::DateTime<UTC>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Pomodoro {
    id: Uuid,
    start: PomTime,
    end: Option<PomTime>,
    task: Option<String>,

    // Put tables last
    kind: PomodoroKind,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pomodoros {
    poms: Vec<Pomodoro>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
pub enum PomodoroKind {
    RegularWork,
    ShortBreak,
    LongBreak,
}

impl Pomodoros {
    pub fn new() -> Self {
        Pomodoros { poms: vec![] }
    }

    pub fn to_string(&self) -> Result<String, ()> {
        match toml::to_string(self) {
            Ok(out) => Ok(out),
            Err(_) => Err(()),
        }
    }
}

impl Pomodoro {
    pub fn new(task: Option<&str>, kind: PomodoroKind) -> Self {
        Pomodoro {
            id: Uuid::new_v4(),
            start: UTC::now(),
            end: None,
            task: match task {
                Some(taskstr) => Some(taskstr.to_owned()),
                None => None,
            },
            kind: kind,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::DateTime;
    use std::str::FromStr;

    #[test]
    fn history_serialize() {
        let history = r#"
            [[poms]]
            id = "424f33fe-9b60-48bf-8c63-974e23a5d7de"
            start = "2017-03-12T00:49:59.626061638Z"
            task = "People"

            [poms.kind]
            type = "RegularWork"
            [[poms]]
            id = "424f33fe-9b60-48bf-8c63-974e23a5d7de"
            start = "2017-03-12T00:49:59.626061638Z"
            task = "Like"

            [poms.kind]
            type = "RegularWork"
            [[poms]]
            id = "424f33fe-9b60-48bf-8c63-974e23a5d7de"
            start = "2017-03-12T00:49:59.626061638Z"
            task = "Butts"

            [poms.kind]
            type = "RegularWork"
        "#;

        // TODO: actually check the contents of the deserialized structure, not just failures
        let _: Pomodoros = toml::from_str(&history).unwrap();
    }

    #[test]
    fn history_deserialize() {
        let history = Pomodoros {
            poms: vec![Pomodoro {
                           start: DateTime::from_str(&"1996-12-19T16:39:57-08:00").unwrap(),
                           end: None,
                           task: Some("Fighting the man".to_owned()),
                           kind: PomodoroKind::RegularWork,
                           id: Uuid::nil(),
                       },
                       Pomodoro {
                           start: DateTime::from_str(&"1996-12-19T16:39:57-08:00").unwrap(),
                           end: None,
                           task: Some("Fighting the law".to_owned()),
                           kind: PomodoroKind::ShortBreak,
                           id: Uuid::nil(),
                       }],
        };

        let output = toml::to_string(&history).unwrap();
        assert_eq!(output,
                   "[[poms]]\nid = \"00000000-0000-0000-0000-000000000000\"\nstart = \
                    \"1996-12-20T00:39:57Z\"\ntask = \"Fighting the man\"\n\n[poms.kind]\ntype = \
                    \"RegularWork\"\n[[poms]]\nid = \
                    \"00000000-0000-0000-0000-000000000000\"\nstart = \
                    \"1996-12-20T00:39:57Z\"\ntask = \"Fighting the law\"\n\n[poms.kind]\ntype = \
                    \"ShortBreak\"\n");
    }
}
