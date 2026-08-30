// Kata solution: walk a recursive generic list without recursion.

struct ListNode<T> {
    data: T,
    next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    fn new(data: T) -> Self {
        Self { data, next: None }
    }

    fn push_front(self, data: T) -> Self {
        Self { data, next: Some(Box::new(self)) }
    }

    // `as_deref()` turns &Option<Box<ListNode<T>>> into Option<&ListNode<T>>,
    // which is the whole trick: the loop variable never owns anything.
    fn values(&self) -> Vec<&T> {
        let mut out = Vec::new();
        let mut current = Some(self);
        while let Some(node) = current {
            out.push(&node.data);
            current = node.next.as_deref();
        }
        out
    }
}

fn main() {
    let ballot = ListNode::new("Cara").push_front("Ben").push_front("Ada");
    println!("names   {:?}", ballot.values());
    println!("length  {}", ballot.values().len());

    // The same walk, over a different T. One definition, two lists.
    let scores = ListNode::new(0u8).push_front(3).push_front(5);
    println!("scores  {:?}", scores.values());

    // The list is still there afterwards: values() borrowed, it did not consume.
    println!("first   {}", ballot.values()[0]);
}
