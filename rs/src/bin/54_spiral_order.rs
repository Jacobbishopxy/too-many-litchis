fn main() {
    let v = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];

    println!("result: {:?}", spiral_order(v));
}

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut res = vec![];

    if matrix.is_empty() || matrix[0].is_empty() {
        return res;
    }

    let (mut top, mut left) = (0_usize, 0_usize);
    let (mut right, mut bottom) = (matrix[0].len() - 1, matrix.len() - 1);

    while left <= right && top <= bottom {
        for i in left..=right {
            res.push(matrix[top][i]);
        }

        for i in matrix.iter().take(bottom + 1).skip(top + 1) {
            res.push(i[right]);
        }
        if left < right && top < bottom {
            for i in ((left + 1)..=(right - 1)).rev() {
                res.push(matrix[bottom][i]);
            }
            for i in ((top + 1)..=bottom).rev() {
                res.push(matrix[i][left]);
            }
        }

        if right == 0 || bottom == 0 {
            break;
        }
        left += 1;
        right -= 1;
        top += 1;
        bottom -= 1;
    }

    res
}
