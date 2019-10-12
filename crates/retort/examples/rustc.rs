use {
    retort::{diagnostic::*, renderer::render, style},
    std::io,
    termcolor::{ColorChoice, StandardStream},
};

fn main() -> io::Result<()> {
    let mut source = include_str!("rustc_source.txt");

    let diagnostic = Diagnostic {
        primary: Annotation {
            span: ("rustc_source.txt", (50, 777)),
            level: Level::Err,
            message: "mismatched types".into(),
        },
        code: Some("E0308".into()),
        secondary: vec![
            Annotation {
                span: ("rustc_source.txt", (55, 69)),
                level: Level::Info,
                message: "expected `Option<String>` because of return type".into(),
            },
            Annotation {
                span: ("rustc_source.txt", (76, 775)),
                level: Level::Err,
                message: "expected enum `std::option::Option`, found ()".into(),
            },
        ]
        .into(),
    };

    let mut w = StandardStream::stdout(ColorChoice::Auto);
    render(&mut w, &mut style::TestStyle, &mut source, &diagnostic)
}
