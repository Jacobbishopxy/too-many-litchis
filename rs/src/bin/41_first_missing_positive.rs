use std::collections::HashSet;

fn main() {
    let res = vec![1, 2, 0];

    println!("result: {}", first_missing_positive(res));
}

pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let nums = nums.into_iter().collect::<HashSet<_>>();
    for i in 1.. {
        if !nums.contains(&i) {
            return i;
        }
    }
    0
}
