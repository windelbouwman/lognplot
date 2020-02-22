use super::util::calc_tick_spacing;
use super::util::ceil_to_multiple_of;
use super::TickLabels;
use chrono::TimeZone;

/// Determine nice date time tick markers.
///
/// Strategies here:
/// - First tick is full date time, subsequent ticks indicate +10s +20s
pub fn calc_date_ticks(begin: f64, end: f64, n_ticks: usize) -> TickLabels {
    let (scale, tick_step) = calc_tick_spacing(end - begin, n_ticks);

    let first_tick: f64 = ceil_to_multiple_of(begin, tick_step);

    let mut ticks = vec![];
    let mut x: f64 = first_tick;
    let mut counter: usize = 0;
    while x < end {
        let is_first: bool = ticks.is_empty();
        let label: String = if is_first {
            let start = f64_to_datetime(x);
            start.format("%Y-%m-%d %H:%M:%S").to_string()
        // start.format("%H:%M:%S").to_string()
        } else {
            let seconds_after_first: f64 = (counter as f64) * tick_step;
            if scale > 0 {
                format!("+{0} s", seconds_after_first)
            } else {
                let digits = (-scale + 1) as usize;
                format!("+{0:.width$} s", seconds_after_first, width = digits)
            }
        };
        ticks.push((x, label));
        x += tick_step;
        counter += 1;
    }
    ticks
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
    use chrono::TimeZone;

    #[test]
    fn date_ticks() {
        let ticks = calc_date_ticks(1581610682.0, 1581610782.0, 7);

        let start = chrono::Local.timestamp(1581610690, 0);
        let label0 = start.format("%Y-%m-%d %H:%M:%S").to_string();

        let expected_ticks = vec![
            (1581610690.0, label0),
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
