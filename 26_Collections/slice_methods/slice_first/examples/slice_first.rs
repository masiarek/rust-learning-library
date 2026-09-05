fn main() {
    let nums = [10, 20, 30];
    let empty: [i32; 0] = [];
    println!("{:?} {:?}", nums.first(), empty.first());

    // nums[0] asserts; first() asks.
    if let Some(x) = nums.first() {
        println!("first is {x}");
    }
    // empty[0] would panic here: index out of bounds.

    // It borrows: x is a &i32 and nums is unchanged.
    let doubled = nums.first().map(|x| x * 2);
    println!("{doubled:?} {nums:?}");

    // Head and tail in one call.
    if let Some((head, tail)) = nums.split_first() {
        println!("head {head}, tail {tail:?}");
    }
}
