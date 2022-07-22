fn main() {
    let v = vec![7, 1, 5, 3, 6, 4];
    println!("{:?}", max_profit(v));

    let v = vec![1, 2, 3, 4, 5];
    println!("{:?}", max_profit(v));

    let v = vec![7, 6, 4, 3, 1];
    println!("{:?}", max_profit(v));

    let v = vec![2, 1, 2, 0, 1];
    println!("{:?}", max_profit(v));
}

fn max_profit(prices: Vec<i32>) -> i32 {
    let n = prices.len();
    let (mut dp0, mut dp1) = (0, -prices[0]);

    for i in prices.iter().take(n).skip(1) {
        dp0 = dp0.max(dp1 + i);
        dp1 = dp1.max(dp0 - i);
    }

    dp0
}
