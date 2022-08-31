#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    match std::str::from_utf8(data) {
        Ok(s) => {
            use parcel_css::stylesheet::{
                StyleSheet, ParserOptions, MinifyOptions, PrinterOptions
            };

            let mut opts = ParserOptions::default();
            opts.filename = s.to_owned();

            // Parse a style sheet from a string.
            match StyleSheet::parse(
                r#"
                .foo {
                  color: red;
                }

                .bar {
                  color: red;
                }
                "#,
                opts
            ) {
                Ok(mut stylesheet) => {
                    stylesheet.minify(MinifyOptions::default()).unwrap();
                    stylesheet.to_css(PrinterOptions::default()).unwrap();
                },
                _ => {
                },
            };
        },
        _ => {},
    }
});
