use chrono::{DateTime, Utc};
use influxdb::InfluxDbWriteable;
use serde::{Serialize, Deserialize};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

#[derive(InfluxDbWriteable)]
#[derive(Deserialize, Serialize)]
pub struct Update{
    pub time: DateTime<Utc>,
    #[influxdb(tag)] pub path: String,
    pub source: String,
    pub value: String,
}