use {
    lsp_types::{Location, Position, Range as LSPRange},
    retort::{diagnostic::*, lsp},
    std::ops::Range,
};

fn main() {
    let source = include_str!("rustc_source.txt");

    let position = |i: usize| -> Position {
        let nl = (&source[..i]).rfind('\n').map(|i| i + 1).unwrap_or(0);
        Position {
            line: (&source[..nl]).chars().filter(|&c| c == '\n').count() as u64,
            character: (&source[nl..i])
                .chars()
                .map(|c| c.len_utf16())
                .sum::<usize>() as u64,
        }
    };

    let diagnostic = Diagnostic::build()
        .primary(
            Annotation::build()
                .span(50..777)
                .message("mismatched types")
                .build(),
        )
        .code("E0308")
        .level(Level::Err)
        .secondary(
            Annotation::build()
                .span(55..69)
                .message("expected `Option<String>` because of return type")
                .build(),
        )
        .secondary(
            Annotation::build()
                .span(76..775)
                .message("expected enum `std::option::Option`, found ()")
                .build(),
        )
        .build();

    let out = lsp::render(
        Some(diagnostic),
        Some("retort rustc example".to_string()),
        |range: Range<usize>| {
            Location::new(
                "example:rustc_source.txt".parse().unwrap(),
                LSPRange::new(position(range.start), position(range.end)),
            )
        },
    );

    println!("{:#?}", out);
}
