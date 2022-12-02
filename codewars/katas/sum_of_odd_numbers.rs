// this is embarassing

fn row_sum_odd_numbers(n: i64) -> i64 {
    get_nth(n) + get_nth(n+n-1) / 2 * (n)
}

fn get_nth(n: i64) -> i64 {
    (0.5*(n.pow(2) as f64) + 0.5*(n as f64)) as i64
}

fn main() {
    println!("{}", row_sum_odd_numbers(2));
}
