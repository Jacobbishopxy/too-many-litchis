fn main() {
    let foo = vec![7, 1, 5, 3, 6, 4];
    println!("{:?}", max_profit(foo));

    let foo = vec![1, 2, 3, 4, 5];
    println!("{:?}", max_profit(foo));

    let foo = vec![7, 6, 4, 3, 1];
    println!("{:?}", max_profit(foo));

    let foo = vec![2, 1, 2, 0, 1];
    println!("{:?}", max_profit(foo));
}

fn max_profit(prices: Vec<i32>) -> i32 {
    let mut res = 0;
    for i in 1..prices.len() {
        let diff = prices[i] - prices[i - 1];
        res += if 0 > diff { 0 } else { diff };
    }
    res
}
