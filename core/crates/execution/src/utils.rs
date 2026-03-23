pub fn round_up_to_step(value: f64, step: f64) -> f64 {
    let steps = (value / step).ceil();
    let rounded = steps * step;

    // trim floating-point garbage
    let precision = decimals_from_step(step);
    round_to_precision(rounded, precision)
}

pub fn decimals_from_step(step: f64) -> usize {
    let s = step.to_string();
    s.split('.').nth(1).map(|x| x.len()).unwrap_or(0)
}

pub fn round_to_precision(value: f64, precision: usize) -> f64 {
    let factor = 10_f64.powi(precision as i32);
    (value * factor).round() / factor
}

pub fn round_down_to_step(value: f64, step: f64) -> f64 {
    let steps = (value / step).floor();
    let rounded = steps * step;

    // trim floating-point garbage
    let precision = decimals_from_step(step);
    round_to_precision(rounded, precision)
}
