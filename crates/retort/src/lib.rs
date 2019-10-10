mod annotation;
mod message;
mod render;
mod slice;
mod snippet;
mod span;
pub mod style;

/// Re-export of the syntect. Use this re-export rather than depending directly on syntect.
#[cfg(feature = "syntect")]
pub use ::syntect;

pub use {
    crate::{
        annotation::Annotation,
        message::{DebugAndDisplay, Level, Message},
        render::{render, Mark},
        slice::{Origin, Slice, Spacing},
        snippet::Snippet,
        span::{Span, SpanResolver, SpannedLine},
    },
    termcolor::WriteColor,
};

/// Macro to conditionally include [syntect]-based styling code.
///
/// # Usage
///
/// ```no_run
/// if_syntect_support! ({
///     use retort::syntect::*; // necessary imports _inside_ the macro only!
///     // Any processing of syntect-reliant features, such as Style::Syntect
/// } else {
///
/// });
/// ```
///
///   [`Stylesheet`]: `style::Stylesheet`
///   [syntect]: <https://crates.io/crates/syntect>
#[macro_export]
#[cfg(feature = "syntect")]
macro_rules! if_syntect_support {
    {{$($then:tt)*} else $else:expr} => {{ $($then:tt)* }};
}

#[macro_export]
#[cfg(not(feature = "syntect"))]
macro_rules! if_syntect_support {
    {{$($then:tt)*} else $else:expr} => {{ $else }};
}
