// https://leetcode.com/problems/reverse-linked-list/description/

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: isize,
    next: Option<Box<ListNode>>,
}

fn run(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut current = head;

    while let Some(mut node) = current {
        let next = node.next.take();
        node.next = prev;
        prev = Some(node);
        current = next;
    }

    prev
}

fn rec(node: Option<Box<ListNode>>, prev: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match node {
        Some(mut current) => {
            let next = current.next.take();
            current.next = prev;
            rec(next, Some(current))
        }
        None => prev,
    }
}

fn run2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    rec(head, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_linked_list(values: Vec<isize>) -> Option<Box<ListNode>> {
        let mut current = None;
        for &val in values.iter().rev() {
            current = Some(Box::new(ListNode {
                val,
                next: current,
            }));
        }
        current
    }

    fn to_vec(mut node: Option<Box<ListNode>>) -> Vec<isize> {
        let mut result = Vec::new();
        while let Some(n) = node {
            result.push(n.val);
            node = n.next;
        }
        result
    }

    struct TestCase(Vec<isize>, Vec<isize>);

    #[test]
    fn test() {
        let tests = [
            TestCase(vec![1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1]),
            TestCase(vec![1, 2], vec![2, 1]),
            TestCase(vec![], vec![]),
        ];

        for TestCase(input, expected) in tests {
            // 反復
            let head = to_linked_list(input.clone());
            let res = run(head);
            let res = to_vec(res);
            assert_eq!(res, expected);

            // 再帰
            let head = to_linked_list(input.clone());
            let res = run2(head);
            let res = to_vec(res);
            assert_eq!(res, expected);
        }
    }
}
