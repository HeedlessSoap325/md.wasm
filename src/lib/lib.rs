use pulldown_cmark::{Parser, Options};
use wasm_bindgen::prelude::*; // https://github.com/wasm-bindgen/wasm-bindgen#example

#[wasm_bindgen]
pub fn parse_markdown_to_html(markdown: &str) -> String {
    // Source: https://docs.rs/pulldown-cmark/latest/pulldown_cmark/
    // The code in the docs was adapted to match the usecase
    
    let mut options: Options = pulldown_cmark::Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    let parser: Parser<'_> = pulldown_cmark::Parser::new_ext(markdown, options);

    let mut html: String = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);

    html
}

#[wasm_bindgen]
pub fn word_count(markdown: &str) -> u32 {
    markdown.split_whitespace().count() as u32 + 1
}