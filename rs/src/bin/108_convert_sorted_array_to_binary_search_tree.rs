use std::cell::RefCell;
use std::rc::Rc;

use tml::TreeNode;

fn main() {
    let nums = vec![-10, -3, 0, 5, 9];

    println!("{:?}", sorted_array_to_bst(nums));
}

fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let len = nums.len() as i32;
    sort(&nums, 0, len - 1)
}

fn sort(nums: &Vec<i32>, left: i32, right: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if left > right {
        return None;
    }
    let mid = (left + right) / 2;
    let mut root = TreeNode::new(nums[mid as usize]);
    root.left = sort(nums, left, mid - 1);
    root.right = sort(nums, mid + 1, right);
    Some(Rc::new(RefCell::new(root)))
}
