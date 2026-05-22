<script lang="ts">
	import { onMount } from "svelte"
  	import init, { word_count } from "../wasm/lib";

	onMount(() => {
		init().then(() => {
			console.log(word_count("Hello World!"))
		})
	})
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>
