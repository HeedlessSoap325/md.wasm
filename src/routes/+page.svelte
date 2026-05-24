<script lang="ts">
	import { onMount } from "svelte"
  	import init, { word_count, parse_markdown_to_html } from "$rust/lib";

	const STARTER = `# Hello from WebAssembly! 🦀

This text is parsed by **Rust's pulldown-cmark** library,
compiled to **.wasm** and running right in your browser.

## What you can try

- **Bold**, _italic_, ~~strikethrough~~
- \`inline code\` and code blocks
- Tables, task lists, blockquotes

### Code block

\`\`\`rust
#[wasm_bindgen]
pub fn parse_markdown(input: &str) -> String {
	let parser = Parser::new_ext(input, Options::all());
	let mut html = String::new();
	push_html(&mut html, parser);
	html
}
\`\`\`

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
>  keep the DOM work in JS."`;

	let loaded 	  = $state(false);

	let text      = $state(STARTER);
	let preview   = $state();
	let statWords = $state();
	let statChars = $state();
	let statTime  = $state();

	function render() {
		const t0 = performance.now();

		preview = parse_markdown_to_html(text);
		const words = word_count(text);

		const ms = (performance.now() - t0).toFixed(2);

		statWords = `${words} word${words !== 1 ? 's' : ''}`;
		statChars = `${text.length} chars`;
		statTime  = `${ms}ms`;
	}

	/**
	// ── Optional: draggable divider ─────────────────────────────────────────────
	// This lets you resize the two panes by dragging the divider.
	const divider = document.getElementById('divider');
	const mainEl  = document.querySelector('main');

	let dragging = false;

	divider.addEventListener('mousedown', () => { dragging = true; });

	document.addEventListener('mousemove', (e) => {
	if (!dragging) return;
	const rect  = mainEl.getBoundingClientRect();
	const pct   = ((e.clientX - rect.left) / rect.width) * 100;
	const clamped = Math.max(20, Math.min(80, pct));
	mainEl.style.gridTemplateColumns = `${clamped}fr 1px ${100 - clamped}fr`;
	});

	document.addEventListener('mouseup', () => { dragging = false; });
	**/

	onMount(() => {
		init().then(() => loaded = true);
	});

	$effect(() => {
		if (loaded) {
			render(STARTER);

			editor.addEventListener('input', () => render(editor.value));

			statTime = "Ready";
		}
	});
</script>

<header>
	<div class="logo">
		<span>Markdown Renderer</span>
		<span class="logo-badge">Wasm</span>
	</div>
	<div class="stats">
		<span id="stat-words">{ statWords }</span>
		<span id="stat-chars">{ statChars }</span>
		<span id="stat-time">{ statTime }</span>
	</div>
</header>
  
<main>
	<div class="editor-pane">
		<div class="pane-label">Markdown</div>
		<textarea id="editor" bind:value={text} spellcheck="false" placeholder="Start typing Markdown…"></textarea>
	</div>
  
	<div class="divider" id="divider"></div>
  
	<div class="preview-pane">
		<div class="pane-label">Preview</div>
		<div id="preview">{@html preview}</div>
	</div>
</main>