use crate::DebugAndDisplay;

/// One slice to be annotated.
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
/// there are three slices: one for each bit of code being annotated.
/// The slices in this error are:
///
/// ```
/// # use retort::*;
/// # let line4 = 0..0; let line7 = 0..0; let line9 = 0..0;
/// let slices = &[
///     Slice {
///         span: line4,
///         origin: Some(Origin {
///             file: &"examples/moved_value.rs",
///             pos: Some((4, Some(5))),
///         }),
///         spacing: Spacing::TightBelow,
///         fold: false,
///     },
///     Slice {
///         span: line7,
///         origin: None,
///         spacing: Spacing::Tight,
///         fold: false,
///     },
///     Slice {
///         span: line9,
///         origin: None,
///         spacing: Spacing::Tight,
///         fold: false,
///     },
/// ];
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Slice<'a, Sp> {
    pub span: Sp,
    pub origin: Option<Origin<'a>>,
    pub spacing: Spacing,
    pub fold: bool,
}

/// Spacing around a slice.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Spacing {
    /// Emit a spacing line above and below the snippet.
    Spacious,
    /// Emit a spacing line below the snippet only.
    TightAbove,
    /// Emit a spacing line above the snippet only.
    TightBelow,
    /// Emit no spacing lines.
    Tight,
}

#[derive(Debug, Copy, Clone)]
pub struct Origin<'a> {
    pub file: &'a dyn DebugAndDisplay,
    pub pos: Option<(usize, Option<usize>)>,
}
