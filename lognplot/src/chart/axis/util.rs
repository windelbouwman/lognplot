/// Given the current axis, calculate sensible
/// tick spacing.
pub fn calc_tick_spacing(domain: f64, n_ticks: usize) -> (i32, f64) {
    assert!(n_ticks >= 2);

    let scale = domain.log10().floor();
    let approx = (10.0_f64).powf(-scale) * domain / (n_ticks as f64);

    // Snap to grid:
    // 10, 20, 25, 50
    let options = vec![0.1, 0.2, 0.5, 1.0, 2.0, 5.0];
    let best = options
        .iter()
        .min_by_key(|x| (((*x - approx).abs() * 1_000_000.0) as i64))
        .unwrap();

    trace!(
        "domain: {}, Scale {}, approx: {}, best: {}",
        domain,
        scale,
        approx,
        best
    );

    let tick_width = best * (10.0_f64).powf(scale);
    (scale as i32, tick_width)
}

/// Round the given number to an integer multiple of the given step size.
pub fn ceil_to_multiple_of(x: f64, step: f64) -> f64 {
    let offset = x % step;
    if offset > 0.0 {
        x + step - offset
    } else if offset < 0.0 {
        x - offset
    } else {
        x
    }
}

/// Format a number at with the proper amount of precision given the
/// scale the number is in.
pub fn format_at_scale(value: f64, scale: i32) -> String {
    if scale > 0 {
        format!("{:.0}", value)
    } else {
        let digits = (-scale + 1) as usize;
        format!("{0:.width$}", value, width = digits)
    }
}
