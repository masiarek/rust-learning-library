//! Three spellings make every array, and inference fills in what you leave out.
//!
//!   rustc --edition 2024 writing_an_array_down.rs -o /tmp/wad && /tmp/wad

use std::any::type_name_of_val;
use std::mem::{size_of, size_of_val};
use std::panic;

/// Adds up one row the way a first draft does: an index loop into a `sum` whose
/// type nobody wrote down.
fn index_loop_sum(a: &[u8; 3]) -> u8 {
    let mut sum = 0;
    #[allow(clippy::needless_range_loop)] // the loop this page is about
    for i in 0..a.len() {
        sum += a[i];
    }
    sum
}

fn main() {
    println!("1. Three spellings");
    let listed = [1, 2, 3];
    let annotated: [u8; 3] = [1, 2, 3];
    let repeated = [0u8; 3];
    println!("   [1, 2, 3]              {listed:?}  is {}", type_name_of_val(&listed));
    println!("   let _: [u8; 3] = ...   {annotated:?}  is {}", type_name_of_val(&annotated));
    println!("   [0u8; 3]               {repeated:?}  is {}", type_name_of_val(&repeated));
    let counted: [i32; _] = [4, 5, 6, 7];
    println!("   let _: [i32; _] = [4, 5, 6, 7] is {} -- the compiler counts", type_name_of_val(&counted));

    println!();
    println!("2. Any element type, one per array");
    let floats: [f64; 3] = [0.5, 1.5, 2.5];
    let words: [&str; 4] = ["a", "b", "c", "d"];
    let letters = ['a', 'b'];
    let flags = [true, false, true];
    let cs = ['c'; 3];
    let ds = ["d"; 3];
    println!("   {floats:?} is {}", type_name_of_val(&floats));
    println!("   {words:?} is {}", type_name_of_val(&words));
    println!("   {letters:?} is {}", type_name_of_val(&letters));
    println!("   {flags:?} is {}", type_name_of_val(&flags));
    println!("   {cs:?} is {}", type_name_of_val(&cs));
    println!("   {ds:?} is {}", type_name_of_val(&ds));

    println!();
    println!("3. [value; how many]");
    println!("   [99; 5]     = {:?}", [99; 5]);
    println!("   ['a'; 10]   = {:?}", ['a'; 10]);
    println!("   [true; 1000] has len {} and size_of {} bytes", [true; 1000].len(), size_of::<[bool; 1000]>());
    let empty_names = [const { String::new() }; 3];
    let cloned: [String; 3] = std::array::repeat(String::from("bb"));
    let built: [String; 3] = std::array::from_fn(|i| format!("row {i}"));
    println!("   [const {{ String::new() }}; 3]          = {empty_names:?}");
    println!("   std::array::repeat(String::from(\"bb\")) = {cloned:?}");
    println!("   std::array::from_fn(|i| format!(..))   = {built:?}");
    let one_string = [String::from("only"); 1];
    println!("   [String::from(\"only\"); 1] compiles too: {one_string:?}");
    println!("   (a non-Copy value is fine when there is one copy to make: none)");

    println!();
    println!("4. One suffix types the whole array");
    println!("   [0u8, 0, 0] is {}", type_name_of_val(&[0u8, 0, 0]));
    println!("   [0, 0i32, 0] is {}", type_name_of_val(&[0, 0i32, 0]));
    println!("   [1u8, 2, 3] is {}", type_name_of_val(&[1u8, 2, 3]));

    println!();
    println!("5. Inference reads the neighbours");
    let alone = [1, 2, 3];
    println!("   let alone = [1, 2, 3];  alone is {}", type_name_of_val(&alone));
    let one = [1, 2, 3];
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];
    let arrays = [one, two, blank1, blank2];
    println!("   the same literal beside a [u8; 3]:");
    println!("   one is {}, blank1 is {}", type_name_of_val(&one), type_name_of_val(&blank1));
    println!("   arrays is {}", type_name_of_val(&arrays));
    println!("   size_of_val(&arrays) = {} bytes: twelve u8s, no headers", size_of_val(&arrays));

    println!();
    println!("6. Brackets add a level");
    let a1: [u8; 5] = [0; 5];
    let a2: [u8; 5] = [1; 5];
    let b = [[a1], [a2]];
    let flat = [a1, a2];
    println!("   [[a1], [a2]] is {}", type_name_of_val(&b));
    println!("   [a1, a2]     is {}", type_name_of_val(&flat));

    println!();
    println!("7. Declare now, assign once, later");
    #[allow(clippy::needless_late_init)] // the late form is the point here
    let later: [i64; 5];
    later = [100, 101, 102, 103, 104];
    println!("   let later: [i64; 5]; later = [...];  later = {later:?}");

    println!();
    println!("8. Walking it by reference");
    for a in &arrays {
        let plus_ten: Vec<u8> = a.iter().map(|n| n + 10).collect();
        println!("   a is {:<10} {a:?} -> n + 10 = {plus_ten:?}", type_name_of_val(&a));
    }
    let first = &arrays[0][0];
    println!("   n is {}, and {} + 10 = {}: &u8 implements Add<u8>", type_name_of_val(&first), first, first + 10);

    println!();
    println!("9. The sum nobody typed is a u8");
    println!("   index_loop_sum(&[1, 2, 3]) = {}", index_loop_sum(&[1, 2, 3]));
    let hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let row = [200u8, 100, 0];
    match panic::catch_unwind(|| index_loop_sum(&row)) {
        Ok(sum) => println!("   index_loop_sum(&{row:?}) = {sum}"),
        Err(payload) => {
            let msg = payload.downcast_ref::<&str>().copied().unwrap_or("?");
            println!("   index_loop_sum(&{row:?}) panicked: {msg}");
        }
    }
    panic::set_hook(hook);
    println!("   a release build does not check, and wraps: 200u8.wrapping_add(100) = {}", 200u8.wrapping_add(100));
    let wide: u32 = row.iter().map(|&n| u32::from(n)).sum();
    println!("   widen first: row.iter().map(|&n| u32::from(n)).sum::<u32>() = {wide}");

    println!();
    println!("10. Two functions called from_fn");
    let squares: [usize; 5] = std::array::from_fn(|i| i * i);
    println!("   std::array::from_fn(|i| i * i) -> {squares:?}  (index in, element out, N from the type)");
    let mut n = 0;
    let countdown: Vec<i32> = std::iter::from_fn(|| {
        n += 1;
        if n <= 3 { Some(4 - n) } else { None }
    })
    .collect();
    println!("   std::iter::from_fn(|| ..)      -> {countdown:?}  (nothing in, Option out, stops at None)");
}
