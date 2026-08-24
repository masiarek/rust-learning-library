//! Kata solution: the two `dbg!` traps — the alternate flag, and the move.
//!
//!   rustc --edition 2024 what_dbg_does_kata.rs -o /tmp/wddk && /tmp/wddk

use std::fmt;

#[derive(Clone, Debug)] // Debug here is for TRAP 2; the wrappers below write their own
struct Data {
    name: String,
    bones: Vec<String>,
}

// (a) The hand-written Debug that ignores the alternate flag. This is the
//     common shape, and it is wrong in a way nothing warns about.
struct Flat(Data);
impl fmt::Debug for Flat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.0.name)?;
        for bone in &self.0.bones {
            write!(f, " {bone}")?;
        }
        writeln!(f) // and a trailing newline nobody asked for
    }
}

// (b) The same thing built with the Formatter's own helper, which asks.
struct Honest(Data);
impl fmt::Debug for Honest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Data")
            .field("name", &self.0.name)
            .field("bones", &self.0.bones)
            .finish()
    }
}

// (c) Hand-written, but honouring alternate() explicitly — what debug_struct
//     is doing for you underneath.
struct Manual(Data);
impl fmt::Debug for Manual {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "{}\n  bones: {:?}", self.0.name, self.0.bones)
        } else {
            write!(f, "{} {:?}", self.0.name, self.0.bones)
        }
    }
}

fn main() {
    let d = Data { name: "Boxy".into(), bones: vec!["fake1".into(), "fake2".into()] };

    println!("TRAP 1 — a hand-written Debug that never asks f.alternate()\n");
    println!("  Flat with {{:?}}   -> {:?}", Flat(d.clone()));
    println!("  Flat with {{:#?}}  -> {:#?}", Flat(d.clone()));
    println!("  Identical. The alternate flag arrived and the impl ignored it.");
    println!("  dbg! is hard-wired to {{:#?}}, so dbg! gets the flat one too — and the");
    println!("  stray writeln! adds a blank line on top of the newline dbg! prints.\n");

    println!("  Honest with {{:?}}  -> {:?}", Honest(d.clone()));
    println!("  Honest with {{:#?}} ->");
    println!("{:#?}", Honest(d.clone()));
    println!("  f.debug_struct() checks the flag for you. Reach for it first.\n");

    println!("  Manual with {{:?}}  -> {:?}", Manual(d.clone()));
    println!("  Manual with {{:#?}} -> {:#?}", Manual(d.clone()));
    println!("  ...and if you must write it by hand, f.alternate() is the question.\n");

    println!("TRAP 2 — dbg! moves a non-Copy argument\n");
    println!("  dbg!(d);        // moves d in. It hands the value back...");
    println!("  println!(\"{{}}\", d.name);   // ...but you did not catch it: E0382");
    println!();
    println!("  Two fixes, and they mean different things:");
    let d = dbg!(d); //        catch the value back
    println!("    let d = dbg!(d);   rebind — d is alive because you took the return");
    dbg!(&d); //                borrow instead
    println!("    dbg!(&d);          borrow — nothing moved at all, and the printed");
    println!("                       label shows `&d` rather than `d`");
    println!("  Reach for the borrow. It reads as an inspection, which is what it is.");
    println!("  still own it: {} with {} bones", d.name, d.bones.len());

    println!("\n(Both dbg! calls above wrote to stderr, so they are absent from this");
    println!("page's recorded output — see section 3 of the lesson.)");
}
