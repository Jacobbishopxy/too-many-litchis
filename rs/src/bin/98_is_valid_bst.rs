use std::cell::RefCell;
use std::rc::Rc;
use tml::TreeNode;

fn main() {
    let root = TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
        }))),
    };
    let root = Some(Rc::new(RefCell::new(root)));

    println!("{:?}", is_valid_bst(root));
}

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    recursive(&root, i64::MIN, i64::MAX)
}

fn recursive(root: &Option<Rc<RefCell<TreeNode>>>, lower: i64, upper: i64) -> bool {
    match root {
        Some(r) => {
            let val = r.borrow().val as i64;
            if val <= lower || val >= upper {
                false
            } else {
                recursive(&r.borrow().left, lower, val) && recursive(&r.borrow().right, val, upper)
            }
        }
        None => true,
    }
}
