//! The same struct twice: once under each version of the derive.

pub mod call_site {
    #[derive(summary_derive::SummaryCallSite)]
    pub struct Comment {
        pub author: String,
        pub text: String,
    }
}

pub mod mixed_site {
    #[derive(summary_derive::Summary)]
    pub struct Comment {
        pub author: String,
        pub text: String,
    }
}
