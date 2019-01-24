use chrono::DateTime;
use chrono::offset::Utc;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Weight(f64);

struct WeightLogEntry {
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

    fn weight(&self) -> Weight {
        self.weight
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
}


struct WeightLog(Vec<WeightLogEntry>);

impl WeightLog {
    fn new() -> Self {
        WeightLog(Vec::new())
    }
    
    fn len(&self) -> usize {
        self.0.len()
    }

    fn insert(&mut self, entry: WeightLogEntry) {
        let check_if_sort_required = self.0.last()
            .map(WeightLogEntry::timestamp)
            .filter(|&ts| ts > entry.timestamp());

        self.0.push(entry);
        if let Some(_) = check_if_sort_required {
            self.0.sort_unstable_by_key(WeightLogEntry::timestamp);
        }
    }

    fn as_slice(&self) -> &[WeightLogEntry] {
        &self.0.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

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
}
