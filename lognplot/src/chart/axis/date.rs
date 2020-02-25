use super::util::{calc_tick_spacing, ceil_to_multiple_of, format_at_scale};
use super::TickLabels;
use chrono::TimeZone;

/// Determine nice date time tick markers.
///
/// Strategies here:
/// - First tick is full date time, subsequent ticks indicate +10s +20s
pub fn calc_date_ticks(begin: f64, end: f64, n_ticks: usize) -> (String, TickLabels) {
    let (scale, tick_step) = calc_tick_spacing(end - begin, n_ticks);

    let first_tick: f64 = ceil_to_multiple_of(begin, tick_step);

    let mut ticks = vec![];
    let mut x: f64 = first_tick;
    let mut counter: usize = 0;

    let start = f64_to_datetime(x);
    let prefix = start.format("%Y-%m-%d %H:%M:%S%.9f").to_string();

    while x < end {
        let seconds_after_first: f64 = (counter as f64) * tick_step;
        let label: String = format!("+{0} s", format_at_scale(seconds_after_first, scale));
        ticks.push((x, label));
        x += tick_step;
        counter += 1;
    }
    (prefix, ticks)
}

fn f64_to_datetime(timestamp: f64) -> chrono::DateTime<chrono::Local> {
    let seconds = timestamp.trunc() as i64;
    let nanos = (timestamp.fract() * 1e9) as u32;
    chrono::Local.timestamp(seconds, nanos)
}

#[cfg(test)]
pub mod tests {
    use super::super::tests::compare_ticks;
    use super::calc_date_ticks;

    #[test]
    fn date_ticks() {
        let (_, ticks) = calc_date_ticks(1581610682.0, 1581610782.0, 7);

        let expected_ticks = vec![
            (1581610690.0, "+0 s".to_string()),
            (1581610700.0, "+10 s".to_string()),
            (1581610710.0, "+20 s".to_string()),
            (1581610720.0, "+30 s".to_string()),
            (1581610730.0, "+40 s".to_string()),
            (1581610740.0, "+50 s".to_string()),
            (1581610750.0, "+60 s".to_string()),
            (1581610760.0, "+70 s".to_string()),
            (1581610770.0, "+80 s".to_string()),
            (1581610780.0, "+90 s".to_string()),
        ];
        println!("{:?}", ticks);
        compare_ticks(ticks, expected_ticks);
    }
}
