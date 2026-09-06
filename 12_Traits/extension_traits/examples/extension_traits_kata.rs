//! Kata: one extension trait, two methods, and only one of them reachable
//! through a dot.
//!
//! `middle` collides with nothing, so `v.middle()` finds it. `first` collides
//! with the inherent `[T]::first`, so `v.first()` never reaches this file — and
//! nothing in the build says so. The long spelling is the only way in.

trait SliceExt<T> {
    /// The element at the halfway index. `[10, 20, 30]` gives `20`;
    /// an even-length slice gives the first of the middle pair's right half.
    fn middle(&self) -> Option<&T>;

    /// Deliberately named to collide. Returns a value the inherent method
    /// could not possibly return, so the transcript below is unambiguous
    /// about which one ran.
    fn first(&self) -> usize;
}

impl<T> SliceExt<T> for [T] {
    fn middle(&self) -> Option<&T> {
        self.get(self.len() / 2)
    }

    fn first(&self) -> usize {
        999
    }
}

fn main() {
    let odd = [10, 20, 30];
    let even = [10, 20, 30, 40];
    let empty: [i32; 0] = [];

    println!("middle — no collision, so the dot finds it:");
    println!("  {:<16}.middle() = {:?}", format!("{odd:?}"), odd.middle());
    println!("  {:<16}.middle() = {:?}", format!("{even:?}"), even.middle());
    println!("  {:<16}.middle() = {:?}", format!("{empty:?}"), empty.middle());

    println!("\nfirst — collides, so the dot never gets here:");
    println!("  odd.first()           = {:?}", odd.first());
    println!("  SliceExt::first(&odd) = {}", SliceExt::first(&odd[..]));

    println!("\nThe give-away is the TYPE, not a warning:");
    let by_dot = odd.first();
    let by_trait = SliceExt::first(&odd[..]);
    println!("  by_dot   is {}", std::any::type_name_of_val(&by_dot));
    println!("  by_trait is {}", std::any::type_name_of_val(&by_trait));
    println!("\n  Two calls, one name, one value — and no diagnostic anywhere.");
}
