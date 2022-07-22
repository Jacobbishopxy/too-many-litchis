use std::cmp::max;

fn main() {
    let v1 = vec![2, 3, 1, 1, 4];
    println!("{}", can_jump(v1));

    let v2 = vec![3, 2, 1, 0, 4];
    println!("{}", can_jump(v2));
}

pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut m = 0;
    for (i, v) in nums.iter().enumerate() {
        if i > m {
            return false;
        }
        m = max(m, i + *v as usize);
    }
    true
}
