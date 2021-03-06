use tml::ListNode;

fn main() {
    let res = ListNode::from_vec(vec![1, 2, 3, 4, 5]);

    println!("result: {:?}", res);
    println!("result: {:?}", remove_nth_from_end(res, 2));
}

fn remove(head: Option<Box<ListNode>>, n: i32) -> (Option<Box<ListNode>>, i32) {
    match head {
        Some(tail) => {
            let (rest, tail_idx) = remove(tail.next, n);
            let res = if tail_idx == n {
                rest
            } else {
                Some(Box::new(ListNode {
                    val: tail.val,
                    next: rest,
                }))
            };
            (res, tail_idx + 1)
        }
        None => (None, 1),
    }
}

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let (res, _) = remove(head, n);
    res
}
