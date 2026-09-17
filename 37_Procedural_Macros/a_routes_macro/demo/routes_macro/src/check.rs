//! Everything `routes!` checks before it writes a line of code. Each problem
//! becomes a `syn::Error` on the token responsible, and all of them are
//! returned together, so one build reports every mistake in the table.

use crate::route::{Route, Segment, parameters, segments};
use syn::Error;

pub fn check(routes: &[Route]) -> syn::Result<()> {
    let mut errors = Vec::new();
    // Each route's method and path shape, `GET users/{}`, in table order.
    let mut shapes: Vec<(String, &Route)> = Vec::new();
    for route in routes {
        let path = route.path.value();
        let segments = match segments(&path) {
            Ok(segments) => segments,
            Err(message) => {
                errors.push(Error::new(route.path.span(), message));
                continue;
            }
        };
        let taken = match parameters(&route.handler) {
            Ok(taken) => taken,
            Err(error) => {
                errors.push(error);
                continue;
            }
        };
        let handler = &route.handler.sig.ident;
        for segment in &segments {
            if let Segment::Param(name) = segment
                && !taken.iter().any(|ident| *ident == name)
            {
                let message = format!("the path captures `{{{name}}}`, but `{handler}` takes no `{name}`");
                errors.push(Error::new(route.path.span(), message));
            }
        }
        for ident in &taken {
            if !segments.iter().any(|s| matches!(s, Segment::Param(name) if *ident == name)) {
                let message = format!("`{ident}` is not captured by {path:?}; add `{{{ident}}}` to the path");
                errors.push(Error::new(ident.span(), message));
            }
        }
        let shape = segments.iter().map(|segment| match segment {
            Segment::Literal(text) => text.as_str(),
            Segment::Param(_) => "{}",
        });
        let shape = format!("{} {}", route.method, shape.collect::<Vec<_>>().join("/"));
        if let Some((_, earlier)) = shapes.iter().find(|(seen, _)| *seen == shape) {
            let message = format!("this route matches the same requests as {:?}", earlier.path.value());
            let mut error = Error::new(route.path.span(), message);
            error.combine(Error::new(earlier.path.span(), "the earlier route is here"));
            errors.push(error);
        } else {
            shapes.push((shape, route));
        }
    }
    let mut errors = errors.into_iter();
    match errors.next() {
        None => Ok(()),
        Some(mut first) => {
            first.extend(errors);
            Err(first)
        }
    }
}
