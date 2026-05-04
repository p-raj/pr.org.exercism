use time::PrimitiveDateTime as DateTime;
use time::Duration;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    const GIGA_SEC: i64 = 1_000_000_000;
    let duration_to_add = Duration::new(GIGA_SEC, 0);
    start + duration_to_add
}
