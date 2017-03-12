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
    poms: Vec<Pomodoro>
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
        Pomodoros {
            poms: vec!(),
        }
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
        use self::PomodoroKind::*;

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