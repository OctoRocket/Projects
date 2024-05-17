fn parallel_resistance(input: Vec<f64>) -> f64 {
    input
        .into_iter()
        .map(|num| num.powi(-1))
        .sum::<f64>()
        .powi(-1)
}

fn series_resistance(input: Vec<f64>) -> f64 {
    input.into_iter().sum()
}

fn main() {
    println!(
        "{}",
        series_resistance(vec![
            1000.0,
            parallel_resistance(vec![2000.0, 3000.0]),
            4000.0,
        ])
    );
}
