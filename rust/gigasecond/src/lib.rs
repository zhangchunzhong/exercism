use chrono::{DateTime, Utc, Duration};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let gs:i64   = 10i64.pow(9);
    let dt = start + Duration::seconds(gs);
    dt
}
