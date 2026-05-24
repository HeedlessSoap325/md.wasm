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

	let dragging = false;
	let mainEl = $state();
	let dividerEl = $state();
	let editorWidthFR = $state(50);

	function render() {
		console.log("test")
		const t0 = performance.now();

		preview = parse_markdown_to_html(text);
		const words = word_count(text);

		const ms = (performance.now() - t0).toFixed(2);

		statWords = `${words} word${words !== 1 ? 's' : ''}`;
		statChars = `${text.length} chars`;
		statTime  = `${ms}ms`;
	}

	function dividerMouseDown(e) {
		if (e.target === dividerEl) {
			e.preventDefault();
			dragging = true;
		}
	}

	function dividerMouseMove(e) {
		if (!dragging) return;

		const rect  = mainEl.getBoundingClientRect();
		const pct   = ((e.clientX - rect.left) / rect.width) * 100;
		editorWidthFR = Math.max(20, Math.min(80, pct));
	}

	function dividerMouseUp() {
		dragging = false;
	}

	onMount(() => {
		init().then(() => {
			loaded = true;
			render();
			statTime = "Ready";
		});
	});
</script>
  
<main bind:this={mainEl} onmousedown={dividerMouseDown} onmousemove={dividerMouseMove} onmouseup={dividerMouseUp} aria-hidden="true" style="--editorWidthFR: {editorWidthFR}fr; --previewWidthFR: {100 - editorWidthFR}fr; ">
	<div class="editor-pane">
		<div class="pane-label">
			<span>Markdown</span>
		</div>
		<textarea id="editor" oninput={render} bind:value={text} spellcheck="false" placeholder="Start typing Markdown…"></textarea>
	</div>
  
	<div bind:this={dividerEl} id="divider"></div>
  
	<div class="preview-pane">
		<div class="pane-label">
			<span>Preview</span>

			<div class="stats">
				<span id="stat-words">{ statWords }</span>
				<span id="stat-chars">{ statChars }</span>
				<span id="stat-time">{ statTime }</span>
			</div>
		</div>

		<div id="preview" class="markdown-body">{@html preview}</div>
	</div>
</main>

<style>
	@import "$lib/assets/github-markdown-dark.css";

    *, *::before, *::after { 
		box-sizing: border-box; 
		margin: 0; 
		padding: 0; 
	}

    :root {
		--bg:       #0f1117;
		--surface:  #1a1d27;
		--border:   #2a2d3a;
		--accent:   #7c6af7;
		--text:     #e2e4ef;
		--muted:    #6b6f85;
		--font-mono: 'JetBrains Mono', 'Fira Code', 'Cascadia Code', monospace;
		--font-sans: 'Inter', system-ui, sans-serif;
		--divider-width: 5px;
		--pane-margin-inline: 16px;
		--pane-margin-incol: 8px;
    }

	:global(html, body) {
		height: 100%;
	}

	:global(body) {
		font-family: var(--font-sans);
		background: var(--bg);
		color: var(--text);
		display: flex;
		flex-direction: column;
		overflow: hidden;
		padding: 0;
		margin: 0;
	}

    main {
		display: grid;
		grid-template-columns: var(--editorWidthFR) var(--divider-width) var(--previewWidthFR); /* editorWidthFR & previewWidthFR is passed from TS */
		flex: 1;
		overflow: hidden;
    }

    #divider {
		width: var(--divider-width);
		background: var(--border);
		cursor: col-resize;
		transition: background 0.2s;
		border-radius: var(--divider-width);
    }

    #divider:hover { 
		background: var(--accent); 
	}

    .editor-pane {
		display: flex;
		flex-direction: column;
		overflow: hidden;
    }

    .pane-label {
		padding: 8px 16px;
		font-size: 11px;
		font-weight: 600;
		letter-spacing: 0.08em;
		text-transform: uppercase;
		color: var(--muted);
		border-bottom: 1px solid var(--border);
		flex-shrink: 0;
		display: flex;
		flex-direction: row;
		justify-content: space-between;
    }

	.stats {
		font-size: 12px;
		color: var(--muted);
		display: flex;
		gap: 16px;
    }

    .stats span { 
		transition: color 0.2s; 
	}

    #editor {
		flex: 1;
		width: calc(100% - 2 * var(--pane-margin-inline));
		resize: none;
		border: none;
		outline: none;
		background: var(--surface);
		color: var(--text);
		font-family: var(--font-mono);
		font-size: 14px;
		line-height: 1.7;
		margin: var(--pane-margin-incol) var(--pane-margin-inline);
		padding: 10px 12px;
		padding-bottom: calc(10 * var(--pane-margin-incol));
		tab-size: 2;
		caret-color: var(--accent);
		scrollbar-width: none;
    }

    #editor::selection { 
		background: rgba(124, 106, 247, 0.3); 
	}

    .preview-pane {
		display: flex;
		flex-direction: column;
		overflow: hidden;
    }

	#preview {
		overflow: scroll;
		scrollbar-width: none;
		width: calc(100% - 2 * var(--pane-margin-inline));
		margin: var(--pane-margin-incol) var(--pane-margin-inline);
		padding-bottom: calc(10 * var(--pane-margin-incol));
	}
</style>