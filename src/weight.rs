use chrono::DateTime;
use chrono::offset::Utc;
use csv::Reader;
use failure::Error;
use serde::Deserialize;

use std::fmt;
use std::io::Read;

#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub struct Weight(f64);

impl fmt::Display for Weight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct WeightLogEntry {
    weight: Weight,
    timestamp: DateTime<Utc>,
}

impl WeightLogEntry {
    fn of(weight: Weight) -> Self {
        Self { weight, timestamp: Utc::now() }
    }

    fn at(self, timestamp: DateTime<Utc>) -> Self {
        Self { weight: self.weight, timestamp }
    }

    pub fn weight(&self) -> Weight {
        self.weight
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
}

#[derive(Debug, Deserialize)]
pub struct WeightLog(Vec<WeightLogEntry>);

impl WeightLog {
    pub fn new() -> Self {
        WeightLog(Vec::new())
    }

    pub fn from_csv(reader: impl Read) -> Result<Self, Error> {
        let entries = Reader::from_reader(reader)
            .deserialize::<WeightLogEntry>()
            .collect::<Result<_, _>>()?;

        Ok(WeightLog(entries))
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn insert(&mut self, entry: WeightLogEntry) {
        let sort_required = self.0.last()
            .map(WeightLogEntry::timestamp)
            .filter(|&ts| ts > entry.timestamp());

        self.0.push(entry);
        if let Some(_) = sort_required {
            self.0.sort_unstable_by_key(WeightLogEntry::timestamp);
        }
    }

    pub fn as_slice(&self) -> &[WeightLogEntry] {
        &self.0.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use std::io::BufReader;

    #[test]
    fn weight_log_entries_can_be_constructed_with_just_weight() {
        let weight = Weight(76.0);
        let now = Utc::now();

        let entry = WeightLogEntry::of(weight);

        assert_eq!(entry.weight(), weight);
        assert!(now - entry.timestamp() < Duration::milliseconds(100));
    }

    #[test]
    fn weight_log_entries_can_be_constructed_with_weight_and_timestamp() {
        let weight = Weight(76.0);
        let timestamp = Utc::now();

        let entry = WeightLogEntry::of(weight).at(timestamp);

        assert_eq!(entry.weight(), weight);
        assert_eq!(entry.timestamp(), timestamp);
    }

    #[test]
    fn weight_log_entries_can_be_inserted() {
        let mut log = WeightLog::new();

        log.insert(WeightLogEntry::of(Weight(76.0)));

        assert_eq!(log.len(), 1);
    }

    #[test]
    fn weight_log_entries_are_inserted_in_ascending_date_order() {
        let weight = Weight(76.0);
        let now = Utc::now();
        let older_timestamp = now - Duration::days(1);
        let mut log = WeightLog::new();
        log.insert(WeightLogEntry::of(weight).at(now));

        log.insert(WeightLogEntry::of(weight).at(older_timestamp));

        let timestamps = log.as_slice().iter()
            .map(WeightLogEntry::timestamp)
            .collect::<Vec<_>>();
        assert_eq!(&timestamps, &[older_timestamp, now]);
    }

    #[test]
    fn weight_log_can_be_constructed_empty() {
        let log = WeightLog::new();

        assert_eq!(log.len(), 0);
    }

    #[test]
    fn weight_log_can_be_construct_with_valid_csv() {
        let csv = [
            "weight,timestamp",
            "76.0,2019-01-01T00:06:00+00:00",
            "75.0,2019-01-02T00:06:00+00:00",
            "74.0,2019-01-03T00:06:00+00:00",
        ].join("\n");
        let reader = BufReader::new(csv.as_bytes());

        let log = WeightLog::from_csv(reader).unwrap();

        assert_eq!(log.len(), 3);
    }

    #[test]
    fn weight_log_cannot_be_constructed_with_csv_missing_header_fields() {
        let csv = [
            "76.0,2019-01-01T00:06:00+00:00",
            "75.0,2019-01-02T00:06:00+00:00",
            "74.0,2019-01-03T00:06:00+00:00",
        ].join("\n");
        let reader = BufReader::new(csv.as_bytes());

        let log = WeightLog::from_csv(reader);

        assert!(log.is_err());
    }

    #[test]
    fn weight_log_cannot_be_constructed_with_invalid_weight_values() {
        let csv = [
            "weight,timestamp",
            "76.0.0,2019-01-01T00:06:00+00:00",
        ].join("\n");
        let reader = BufReader::new(csv.as_bytes());

        let log = WeightLog::from_csv(reader);

        assert!(log.is_err());
    }

    #[test]
    fn weight_log_cannot_be_constructed_with_invalid_timestamp_values() {
        let csv = [
            "weight,timestamp",
            "76.0,2019-01-01T00:06:00+00:00:00",
        ].join("\n");
        let reader = BufReader::new(csv.as_bytes());

        let log = WeightLog::from_csv(reader);

        assert!(log.is_err());
    }
}
