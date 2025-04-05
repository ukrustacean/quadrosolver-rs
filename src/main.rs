fn main() {
    println!("{:?}", solve(2., 1., -3.));
}

fn solve(a: f64, b: f64, c: f64) -> [f64; 2] {
    let d = (b.powi(2) - 4.0 * a * c).sqrt();
    [
        (-b + d) / 2.0 / a,
        (-b - d) / 2.0 / a,
    ]
}