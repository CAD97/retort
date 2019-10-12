use {
    lsp_types::{Location, Position, Range as LSPRange},
    retort::{diagnostic::*, renderer},
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

    let diagnostic = Diagnostic {
        primary: Annotation {
            span: (50, 777),
            level: Level::Err,
            message: "mismatched types".into(),
        },
        code: Some("E0308".into()),
        secondary: vec![
            Annotation {
                span: (55, 69),
                level: Level::Info,
                message: "expected `Option<String>` because of return type".into(),
            },
            Annotation {
                span: (76, 775),
                level: Level::Err,
                message: "expected enum `std::option::Option`, found ()".into(),
            },
        ]
        .into(),
    };

    let out = renderer::lsp(
        Some(diagnostic),
        Some("retort rustc example"),
        |(start, end)| {
            Location::new(
                "example:rustc_source.txt".parse().unwrap(),
                LSPRange::new(position(start), position(end)),
            )
        },
    );

    println!("{:#?}", out);
}
