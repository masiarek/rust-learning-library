// A generic type that contains itself — and what the Box and the Option each buy.

#[derive(Debug)]
struct ListNode<T> {
    data: T,
    next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    fn new(data: T) -> Self {
        Self { data, next: None }
    }

    // Put a new node in front of the list we already have.
    fn push_front(self, data: T) -> Self {
        Self { data, next: Some(Box::new(self)) }
    }

    fn len(&self) -> usize {
        let mut n = 0;
        let mut current = Some(self);
        while let Some(node) = current {
            n += 1;
            current = node.next.as_deref();
        }
        n
    }
}

// The hand-rolled alternative to Option, written out.
enum NextNode<T> {
    Next(Box<OtherNode<T>>),
    End,
}

struct OtherNode<T> {
    data: T,
    next: NextNode<T>,
}

// Boxing the payload as well — what the extra indirection costs.
struct BoxedData<T> {
    data: Box<T>,
    next: Option<Box<BoxedData<T>>>,
}

fn main() {
    let ballot = ListNode::new("Cara").push_front("Ben").push_front("Ada");

    let mut current = Some(&ballot);
    let mut rank = 1;
    while let Some(node) = current {
        println!("  {rank}. {}", node.data);
        current = node.next.as_deref();
        rank += 1;
    }
    println!("length {}", ballot.len());
    println!();

    // The Option costs nothing: a null pointer is not a valid Box, so None
    // reuses that bit pattern instead of adding a tag.
    println!("size_of::<Box<ListNode<i32>>>()          {}", size_of::<Box<ListNode<i32>>>());
    println!("size_of::<Option<Box<ListNode<i32>>>>()  {}", size_of::<Option<Box<ListNode<i32>>>>());
    println!("size_of::<NextNode<i32>>()               {}", size_of::<NextNode<i32>>());
    println!();

    // One node: the payload inline, plus one pointer to the rest.
    println!("size_of::<ListNode<i32>>()               {}", size_of::<ListNode<i32>>());
    println!("size_of::<ListNode<[u8; 64]>>()          {}", size_of::<ListNode<[u8; 64]>>());
    println!("size_of::<BoxedData<[u8; 64]>>()         {}", size_of::<BoxedData<[u8; 64]>>());
    println!();

    // Two instantiations of one definition, and they are unrelated types.
    let scores = ListNode::new(5u8).push_front(3);
    println!("ListNode<&str> of {}, ListNode<u8> of {}", ballot.len(), scores.len());

    let end = OtherNode { data: 0u8, next: NextNode::End };
    let two = OtherNode { data: 1u8, next: NextNode::Next(Box::new(end)) };
    println!("the hand-rolled spelling walks the same way: {} then {}", two.data,
        match &two.next {
            NextNode::Next(node) => node.data,
            NextNode::End => u8::MAX,
        });

    let boxed = BoxedData { data: Box::new([0u8; 64]), next: None };
    println!("BoxedData holds {} bytes behind one more pointer", boxed.data.len());
    let _ = boxed.next.is_none();
}
