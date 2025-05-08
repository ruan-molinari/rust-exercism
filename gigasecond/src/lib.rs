use time::{Duration, PrimitiveDateTime as DateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    const GIGASECOND: Duration = Duration::SECOND.saturating_mul(1_000_i32.pow(3));
    start.saturating_add(GIGASECOND)
}
