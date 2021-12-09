use std::cell::RefCell;
use std::rc::Rc;

use tml::TreeNode;

fn main() {
    let root = TreeNode {
        val: -10,
        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    };

    println!("{:?}", max_path_sum(Some(Rc::new(RefCell::new(root)))));
}

fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut num = i32::MIN;

    fn max_gain(max_sum: &mut i32, root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(rt) => {
                // 计算左边分支最大值，如果为负则不选择
                let left_gain = 0.max(max_gain(max_sum, rt.borrow().left.clone()));
                // 同理计算右边分支
                let right_gain = 0.max(max_gain(max_sum, rt.borrow().right.clone()));
                // 比较现有的最大值与计算后的最大值之和，将其赋值给 max_sum
                *max_sum = (*max_sum).max(rt.borrow().val + left_gain + right_gain);
                // 返回当前节点的值与左右分支中最大值之和
                rt.borrow().val + left_gain.max(right_gain)
            }
            None => 0,
        }
    }

    max_gain(&mut num, root);
    num
}
