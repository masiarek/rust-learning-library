//! A growable list of fixed-width rows: `Vec<[T; N]>`.
//!
//!   rustc --edition 2024 vec_of_arrays.rs -o /tmp/voa && /tmp/voa

use std::alloc::{GlobalAlloc, Layout, System};
use std::any::type_name_of_val;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

static ALLOCS: AtomicUsize = AtomicUsize::new(0);

struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

fn main() {
    println!("1. A list that grows, of rows that do not");
    let mut addrs: Vec<[u8; 4]> = Vec::new();
    addrs.push([192, 168, 0, 1]);
    addrs.push([10, 0, 0, 1]);
    addrs.push([127, 0, 0, 1]);
    for addr in &addrs {
        let octets: Vec<String> = addr.iter().map(|o| o.to_string()).collect();
        println!("   {}", octets.join("."));
    }
    println!("   The Vec's length is data; each row's is part of its type.");

    println!();
    println!("2. Vec::new() allocates nothing until you push");
    let before = ALLOCS.load(Relaxed);
    let mut fresh: Vec<[u8; 4]> = Vec::new();
    let empty = ALLOCS.load(Relaxed);
    let cap_empty = fresh.capacity();
    fresh.push([192, 168, 0, 1]);
    fresh.push([10, 0, 0, 1]);
    fresh.push([127, 0, 0, 1]);
    let filled = ALLOCS.load(Relaxed);
    println!("   Vec::new()             -> {} allocations, capacity {}",
             empty - before, cap_empty);
    println!("   three pushes           -> {} allocation, capacity {}",
             filled - empty, fresh.capacity());
    println!("   The first push takes capacity 4 in one block. Rows two and");
    println!("   three cost nothing at all: they were already paid for.");

    println!();
    println!("3. One allocation for a thousand rows, not a thousand and one");
    let before = ALLOCS.load(Relaxed);
    let packed: Vec<[u8; 4]> = vec![[0; 4]; 1000];
    let mid = ALLOCS.load(Relaxed);
    let nested: Vec<Vec<u8>> = vec![vec![0u8; 4]; 1000];
    let after = ALLOCS.load(Relaxed);
    println!("   vec![[0u8; 4]; 1000]      -> {} allocation", mid - before);
    println!("   vec![vec![0u8; 4]; 1000]  -> {} allocations", after - mid);
    println!("   {} rows of four bytes either way; {} in one block, {} in a",
             packed.len(), packed.len(), nested.len());
    println!("   thousand separate ones the outer Vec only points at.");
    let row0 = addrs[0].as_ptr() as usize;
    let row1 = addrs[1].as_ptr() as usize;
    println!("   Row 1 begins {} bytes after row 0 — one block, no gaps.",
             row1 - row0);

    println!();
    println!("4. The rows are Copy, so pushing one does not consume it");
    let loopback = [127, 0, 0, 1];
    addrs.push(loopback);
    println!("   after addrs.push(loopback), loopback is still {loopback:?}");
    let mut rows: Vec<Vec<u8>> = Vec::new();
    let same: Vec<u8> = vec![127, 0, 0, 1];
    rows.push(same);
    // Reading `same` here is error[E0382]: borrow of moved value. Vec<u8> is
    // not Copy, so push took it, and getting it back costs a .clone().
    println!("   the Vec<u8> row moves instead: push consumes it, and reading");
    println!("   it afterwards is error[E0382]: borrow of moved value");
    println!("   addrs holds {} rows, rows holds {}", addrs.len(), rows.len());

    println!();
    println!("5. What each loop hands you");
    let row = &addrs[0];
    println!("   for row in &addrs      -> {}", type_name_of_val(&row));
    for octet in row {
        println!("   for octet in row       -> {}", type_name_of_val(&octet));
        break;
    }
    for octet in *row {
        println!("   for octet in *row      -> {}", type_name_of_val(&octet));
        break;
    }
    println!("   The `*` copies the row out, because [u8; 4] is Copy. Without");
    println!("   it you iterate a &[u8; 4] and get references — which print");
    println!("   identically, so `{{}}` will never tell you which one you have.");

    println!();
    println!("6. Flat, and back again");
    let bytes: &[u8] = addrs.as_flattened();
    println!("   as_flattened(): {} rows -> {} bytes, borrowed, no copy",
             addrs.len(), bytes.len());
    let owned: Vec<u8> = addrs.clone().into_flattened();
    println!("   into_flattened(): the same buffer, retyped — {} bytes", owned.len());

    let mut wire = owned.clone();
    wire.extend_from_slice(&[8, 8]); // a truncated address arrives on the end
    let chunks = wire.chunks_exact(4);
    let leftover = chunks.remainder().to_vec();
    let regrouped: Vec<[u8; 4]> = wire
        .chunks_exact(4)
        .map(|c| c.try_into().expect("chunks_exact(4) yields exactly 4"))
        .collect();
    println!("   {} bytes regroup into {} addresses, with {:?} left over",
             wire.len(), regrouped.len(), leftover);
    println!("   The trip back is fallible and the type says so: a slice has no");
    println!("   length in its type, so try_into is the step that supplies one.");
}
