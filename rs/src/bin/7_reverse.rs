fn main() {
    let res = 1534236469;

    println!("result: {}", reverse(res));
}

pub fn reverse(x: i32) -> i32 {
    x.abs()
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<i32>()
        .unwrap_or(0)
        * if x < 0 { -1 } else { 1 }
}
