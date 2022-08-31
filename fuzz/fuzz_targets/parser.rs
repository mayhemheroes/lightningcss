#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    match std::str::from_utf8(data) {
        Ok(s) => {
            use parcel_css::stylesheet::{
                StyleSheet, ParserOptions, MinifyOptions, PrinterOptions
            };

            // Parse a style sheet from a string.
            match StyleSheet::parse(
                s,
                ParserOptions::default()
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
