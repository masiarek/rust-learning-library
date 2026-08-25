//! Two ways to write "any type that implements this", and what each costs.
//!
//!   rustc --edition 2024 static_vs_dynamic_dispatch.rs -o /tmp/svd && /tmp/svd

trait Processor {
    fn name(&self) -> &'static str;
    fn compute(&self, x: i64, y: i64) -> i64;
}

struct Risc;
struct Cisc;

impl Processor for Risc {
    fn name(&self) -> &'static str {
        "Risc"
    }
    fn compute(&self, x: i64, y: i64) -> i64 {
        x + y
    }
}

impl Processor for Cisc {
    fn name(&self) -> &'static str {
        "Cisc"
    }
    fn compute(&self, x: i64, y: i64) -> i64 {
        x * y
    }
}

// STATIC. The compiler stamps out one copy of this function per concrete P it
// is called with, each one calling that type's `compute` directly.
fn run_static<P: Processor>(p: &P, x: i64) -> i64 {
    p.compute(x, 42)
}

// DYNAMIC. One copy, ever. `p` is a fat pointer, and the call is looked up in
// the vtable at run time.
fn run_dynamic(p: &dyn Processor, x: i64) -> i64 {
    p.compute(x, 42)
}

fn main() {
    let risc = Risc;
    let cisc = Cisc;

    println!("1. Same answers, two dispatch strategies");
    println!("   run_static (&risc, 1) = {:>3}   run_dynamic(&risc, 1) = {:>3}",
        run_static(&risc, 1), run_dynamic(&risc, 1));
    println!("   run_static (&cisc, 2) = {:>3}   run_dynamic(&cisc, 2) = {:>3}",
        run_static(&cisc, 2), run_dynamic(&cisc, 2));

    println!();
    println!("2. The thing generics cannot do: one collection, two types");
    // A Vec<P> holds one concrete P. This holds both, because every element is
    // the same TYPE — a boxed trait object — whatever is inside it.
    let fleet: Vec<Box<dyn Processor>> = vec![Box::new(Risc), Box::new(Cisc)];
    for p in &fleet {
        println!("   {:<4} computes 6 and 7 as {:>2}", p.name(), p.compute(6, 7));
    }

    println!();
    println!("3. What the second pointer costs");
    println!("   &Risc            {:>2} bytes", std::mem::size_of::<&Risc>());
    println!("   &dyn Processor   {:>2} bytes   data pointer + vtable pointer",
        std::mem::size_of::<&dyn Processor>());
    println!("   Box<dyn Proc>    {:>2} bytes   the same pair, owned", std::mem::size_of::<Box<dyn Processor>>());
    println!("   Risc itself      {:>2} bytes   a unit struct: the vtable is shared,",
        std::mem::size_of::<Risc>());
    println!("                            not stored in the value, unlike a C++ vptr.");

    println!();
    println!("4. Static dispatch survives inlining; dynamic dispatch is a real call");
    println!("   Both printed the same numbers above, so correctness is never the");
    println!("   question. The question is one machine-code copy per type and a");
    println!("   direct call, or one copy shared and an indirect one.");
}
