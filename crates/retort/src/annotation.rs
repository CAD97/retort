use crate::Message;

/// An annotation of some span.
///
/// # Example
///
/// In the error message
///
// Please keep in sync with the `moved_value` example!
/// ```text
/// error[E0382]: use of moved value: `x`
///  --> examples/moved_value.rs:4:5
///   |
/// 4 |     let x = vec![1];
///   |         - move occurs because `x` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait
/// 7 |     let y = x;
///   |             - value moved here
/// 9 |     x;
///   |     ^ value used here after move
/// ```
///
/// there are three annotations: one on each line of code.
/// The annotations in this error are:
///
/// ```
/// # use retort::*;
/// # let line4_x = 0..0; let line7_x = 0..0; let line9_x = 0..0;
/// let annotations = &[
///     Annotation {
///         span: line4_x,
///         message: Message {
///             text: &"move occurs because `x` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait",
///             level: Level::Information,
///         },
///     },
///     Annotation {
///         span: line7_x,
///         message: Message {
///             text: "value moved here",
///             level: Level::Information,
///         },
///     },
///     Annotation {
///         span: line9_x,
///         message: Message {
///             text: "value used here after move",
///             level: Level::Error,
///         },
///     },
/// ];
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Annotation<'a, Sp> {
    /// The span to be annotated.
    pub span: Sp,
    /// The message to attach to the span.
    pub message: Message<'a>,
}
