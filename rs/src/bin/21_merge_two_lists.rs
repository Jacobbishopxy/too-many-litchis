use tml::ListNode;

fn main() {
    let res1 = ListNode::from_vec(vec![1, 2, 4]);
    let res2 = ListNode::from_vec(vec![1, 3, 4]);

    println!("result {:?}", merge_two_lists(res1.clone(), res2.clone()));
    println!("result {:?}", Solution2::merge_two_lists(res1, res2));
}

pub fn merge_two_lists(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut res: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
    let mut ptr = &mut res;

    loop {
        match (l1.as_mut(), l2.as_mut()) {
            (Some(v1), Some(v2)) => {
                if v1.val < v2.val {
                    //
                    let next = v1.next.take();
                    ptr.as_mut().unwrap().next = l1.take();
                    l1 = next;
                } else {
                    //
                    let next = v2.next.take();
                    ptr.as_mut().unwrap().next = l2.take();
                    l2 = next;
                }
                ptr = &mut ptr.as_mut().unwrap().next;
            }
            (Some(_), None) => {
                //
                ptr.as_mut().unwrap().next = l1.take();
                break;
            }
            (None, Some(_)) => {
                //
                ptr.as_mut().unwrap().next = l2.take();
                break;
            }
            (None, None) => {
                break;
            }
        }
    }

    res.unwrap().next
}

pub struct Solution2;

impl Solution2 {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (None, r) => r,
            (l, None) => l,
            (Some(mut l), Some(mut r)) => {
                if l.val <= r.val {
                    l.next = Self::merge_two_lists(l.next, Some(r));
                    Some(l)
                } else {
                    r.next = Self::merge_two_lists(Some(l), r.next);
                    Some(r)
                }
            }
        }
    }
}
