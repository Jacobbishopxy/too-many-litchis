use tml::ListNode;

fn main() {
    let head = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
    let k = 2;

    let res = reverse_k_group(head, k);
    let ans = res.unwrap().as_ref().to_vec();

    println!("{:?}", ans);
}

type Obl = Option<Box<ListNode>>;

pub fn reverse_k_group(mut head: Obl, k: i32) -> Obl {
    let mut tail = &mut head;
    for _ in 0..k {
        match tail.as_mut() {
            Some(node) => {
                tail = &mut node.next;
            }
            None => {
                return head;
            }
        }
    }
    let tail = reverse_k_group(tail.take(), k);
    reverse(head, tail)
}

fn reverse(mut head: Obl, mut tail: Obl) -> Obl {
    while let Some(mut node) = head {
        head = node.next.take();

        node.next = tail.take();

        tail = Some(node)
    }
    tail
}
