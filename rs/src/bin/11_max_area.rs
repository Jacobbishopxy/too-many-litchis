use std::cmp::{max, min};
fn main() {
    // let res = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let res = vec![1, 1];

    println!("result: {}", max_area(res));
}

pub fn max_area(height: Vec<i32>) -> i32 {
    let (mut left, mut right) = (0usize, height.len() - 1);

    if right == 0 {
        return 0;
    }
    let mut m = min(height[left], height[right]) * (right - left) as i32;

    while left < right {
        if height[left] > height[right] {
            right -= 1;
        } else {
            left += 1;
        }

        m = max(m, min(height[left], height[right]) * (right - left) as i32);
    }

    m
}
