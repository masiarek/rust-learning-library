//! `Copy` vs `Clone`: one changes what `=` MEANS, the other adds a method.
//!
//!   rustc --edition 2024 copy_vs_clone.rs -o /tmp/cvc && /tmp/cvc

use std::rc::Rc;

// Clone only. Duplicating is possible, but you must ask for it by name.
#[derive(Debug, Clone)]
struct Message {
    author: String,  // String is not Copy, so Message can never be Copy
    bytes: Vec<u8>,
}

// Copy AND Clone. Every field is Copy, and we opted in.
#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

// All-Copy fields, but NOT opted in. This is the case people trip over.
#[derive(Debug, Clone)]
struct Counter {
    count: u32,
}

// ---- Two ways to write it, and the one difference between them -----------
// The same struct twice: a shared reference to something. `&T` is copyable
// whatever T is -- but only one of these two spellings knows that.

#[derive(Debug, Clone, Copy)] // expands to `impl<T: Copy> Copy for Derived<T>`
struct Derived<'a, T>(&'a T);

#[derive(Debug)]
struct Manual<'a, T>(&'a T);
impl<T> Copy for Manual<'_, T> {} // no bound on T at all
impl<T> Clone for Manual<'_, T> {
    fn clone(&self) -> Self { *self } // the only body a Copy type's Clone needs
}

fn bump(x: &mut u32) { *x += 1; }

fn consume_message(m: Message) -> String { format!("{} sent {} bytes", m.author, m.bytes.len()) }
fn consume_point(p: Point) -> i32 { p.x + p.y }
fn consume_counter(c: Counter) -> u32 { c.count }

fn main() {
    println!("1. The difference is what `let b = a;` MEANS");
    let p = Point { x: 7, y: 431 };
    let _also_p = p; //          Copy   -> p is COPIED, and p is still alive
    println!("   Point is Copy: after `let _also = p;`, p is fine -> {p:?}");

    let b = Message { author: "Ada".to_string(), bytes: vec![5, 2, 0] };
    let moved = b; //            not Copy -> b is MOVED, and b is now dead
    println!("   Message is not:  after `let moved = b;`, `b` is E0382");
    println!("                    the value lives on as `moved` -> {moved:?}");
    println!("   Same syntax. Different meaning. The TYPE decides.");

    println!("\n2. Passing to a function is the same question");
    println!("   consume_point(p) = {} (x+y), and p survives -> {p:?}",
             consume_point(p));
    println!("   consume_message(moved) = {:?} and `moved` does not survive",
             consume_message(moved.clone()));
    println!("   ...which is why `.clone()` is in that line at all.");

    println!("\n3. All-Copy fields is NOT enough — you have to opt in");
    let t = Counter { count: 12 };
    println!("   Counter holds one u32 and still is not Copy, because it does");
    println!("   not derive Copy. consume_counter(t) MOVES it:");
    println!("   consume_counter(t) = {}", consume_counter(t));
    println!("   `t` is now dead. Opting in is deliberate: making a type Copy");
    println!("   is a promise to your callers that you cannot quietly take back.");

    println!("\n4. Clone is a method you call; Copy is something the compiler does");
    let original = Message { author: "Ben".to_string(), bytes: vec![4] };
    let duplicate = original.clone(); // explicit, and it allocates
    println!("   original  {original:?}");
    println!("   duplicate {duplicate:?}   <- a second heap allocation, on purpose");
    println!("   `Copy` never allocates: it is a bit-for-bit copy, nothing else.");

    println!("\n5. The three refusals, each with its own code");
    println!("   impl Copy for P {{}} without Clone");
    println!("     error[E0277]: the trait bound `P: Clone` is not satisfied");
    println!("     -> `Copy` requires `Clone`. Always derive both together.");
    println!("   #[derive(Copy)] on a struct holding a String");
    println!("     error[E0204]: the trait `Copy` cannot be implemented for this type");
    println!("       this field does not implement `Copy`");
    println!("   #[derive(Copy)] on a struct that also impls Drop");
    println!("     error[E0184]: `Copy` not allowed on types with destructors");
    println!("     -> a destructor runs once per value; copies would run it twice.");

    println!("\n6. \"Copy is shallow, Clone is deep\" — both halves are false");
    let one = Point { x: 3, y: 100 };
    let mut two = one; //                Copy. If this were a reference to `one`...
    two.y = 999; //                      ...this line would change `one` as well.
    println!("   after copying `one` into `two` and editing `two`:");
    println!("     one  {one:?}");
    println!("     two  {two:?}");
    println!("     same address? {}   <- Copy duplicates the BITS, it never aliases",
             std::ptr::eq(&one, &two));

    let shared = Rc::new(Message { author: "Cara".to_string(), bytes: vec![3, 3] });
    let counted = Rc::clone(&shared); // the most idiomatic clone in Rust, and it copies nothing
    println!("   Rc::clone(&shared):");
    println!("     same allocation? {}   strong_count {}",
             Rc::ptr_eq(&shared, &counted), Rc::strong_count(&shared));
    let independent: Message = (*shared).clone(); // THIS one deep-copies
    println!("     (*shared).clone() gives a separate Vec? {}",
             shared.bytes.as_ptr() != independent.bytes.as_ptr());
    println!("   So `Clone` promises a value you may keep — not a depth.");
    println!("   `Rc` clones a pointer, `String` clones a buffer, and a Copy");
    println!("   type's derived clone is the same memcpy `=` already does:");
    let cloned_point = one.clone();
    println!("     one.clone() = {cloned_point:?}   (no allocation, nothing to free)");
    println!("   The axis is not deep vs shallow. It is WHO ASKS:");
    println!("     Copy  the compiler, silently, at every `=`");
    println!("     Clone you, in writing, at one call site");

    println!("\n7. Two ways to write it — and the one difference");
    println!("   The derive is the simple one, and it writes a bound you did not:");
    println!("     #[derive(Clone, Copy)] struct Derived<T>(&T);");
    println!("       ->  impl<T: Copy> Copy for Derived<T>");
    println!("   That bound is about T, but the field is `&T`, and a shared");
    println!("   reference copies whatever T is. So the derived version refuses");
    println!("   a Derived<Message> that would have been perfectly fine:");
    println!("     error[E0382]: borrow of moved value: `d`");
    println!("       move occurs because `d` has type `Derived<'_, Message>`,");
    println!("       which does not implement the `Copy` trait");
    println!("       note: derived `Clone` adds implicit bounds on type parameters");
    println!("       help: consider manually implementing `Clone` to avoid");
    println!("             undesired bounds");
    println!("   Writing the two impls by hand drops the bound:");
    println!("     impl<T> Copy for Manual<'_, T> {{}}");
    println!("     impl<T> Clone for Manual<'_, T> {{ fn clone(&self) -> Self {{ *self }} }}");
    let held = Message { author: "Dev".to_string(), bytes: vec![1] };
    let m = Manual(&held);
    let m2 = m; // Copy, even though Message is not
    println!("   Manual<Message> copies: {:?} and {:?}", m.0.author, m2.0.author);
    let counted = 431u32;
    let d = Derived(&counted);
    let d2 = d; // Copy, because u32 meets the bound the derive wrote
    println!("   Derived<u32> copies:    {} and {}   <- the bound is met here", d.0, d2.0);
    println!("   And a Copy type's Clone body is always `*self`. There is");
    println!("   nothing else it could be: the compiler already copies the bits.");

    println!("\n8. `&T` is Copy. `&mut T` is not — and the call site hides it");
    let n = 7u32;
    let s = &n;
    let s2 = s; // shared references are Copy
    println!("   shared:  let s = &n; let s2 = s;   both alive -> {s} {s2}");
    let mut m_val = 7u32;
    let r = &mut m_val;
    println!("   unique:  let r = &mut m; let r2 = r;   r is MOVED");
    println!("     error[E0382]: borrow of moved value: `r`");
    println!("       move occurs because `r` has type `&mut u32`,");
    println!("       which does not implement the `Copy` trait");
    println!("   Most people never meet that error, because passing `r` to a");
    println!("   function REBORROWS instead of moving:");
    bump(r);
    bump(r);
    println!("     bump(r); bump(r);   -> {}", *r);
    let r3 = &mut *r; // and you can ask for the reborrow by hand
    bump(r3);
    println!("     &mut *r, bumped     -> {}", *r);
    println!("   Two references, one Copy and one not, for the ownership reason:");
    println!("   `&T` may be duplicated because nobody may write through it.");
}
