# Hello from WebAssembly! 🦀

This text is parsed by **Rust's pulldown-cmark** library,
compiled to **.wasm** and running right in your browser.

## What you can try

- **Bold**, _italic_, ~~strikethrough~~
- `inline code` and code blocks
- Tables, task lists, blockquotes

### Code block

```rust
#[wasm_bindgen]
pub fn parse_markdown(input: &str) -> String {
	let parser = Parser::new_ext(input, Options::all());
	let mut html = String::new();
	push_html(&mut html, parser);
	html
}
```

### Table

| Feature       | JS parser | Wasm parser |
|---------------|-----------|-------------|
| Startup time  | fast      | fast        |
| CPU-heavy ops | slower    | faster      |
| Binary size   | small     | small       |

### Task list

- [x] Set up Rust + wasm-pack
- [x] Write lib.rs
- [ ] Add syntax highlighting
- [ ] Export to file

> "Move the CPU-heavy work to Wasm,
>  keep the DOM work in JS."