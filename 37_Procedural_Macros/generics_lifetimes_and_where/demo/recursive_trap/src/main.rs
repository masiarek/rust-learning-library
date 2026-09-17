use debug_generics::{DebugBoundingFields, DebugBoundingParams};

/// A linked list: the type of `next` mentions `List` itself.
#[derive(DebugBoundingFields)]
struct List<T> {
    value: T,
    next: Option<Box<List<T>>>,
}

/// The same list, bounded the other way.
#[derive(DebugBoundingParams)]
struct ParamList<T> {
    value: T,
    next: Option<Box<ParamList<T>>>,
}

fn main() {
    let by_params = ParamList {
        value: 1,
        next: None,
    };
    let by_fields = List {
        value: 1,
        next: None,
    };
    println!("{by_params:?}");
    println!("{by_fields:?}");
}
