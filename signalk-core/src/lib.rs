use emseries::{Recordable, DateTimeTz};
use serde::{Serialize, Deserialize};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Update{
    pub timestamp: DateTimeTz,
    pub path: String,
    pub source: String,
    pub value: String,
}


impl Recordable for Update {
    fn timestamp(&self) -> DateTimeTz {
        self.timestamp.to_owned()
    }
    fn tags(&self) -> Vec<String> {
        Vec::new()
    }
}

impl Default for Update {
    fn default() -> Self {
        Self {
            path: "".to_string(),
            source: "".to_string(),
            value: "".to_string(),
            timestamp: DateTimeTz(chrono::Utc::now().with_timezone(&chrono_tz::UTC)),
        }
    }
}
