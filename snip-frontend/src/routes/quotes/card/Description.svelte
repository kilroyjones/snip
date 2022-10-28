<script lang="ts">
	export let source: string;
	export let description: string;
</script>

<div id="quote-description"><a href={source} target="_blank">{description}</a></div>

<style>
	#quote-description a {
		color: #2e5077;
	}

	#quote-description a:hover {
		filter: brightness(80%);
	}

	#quote-description {
		font-weight: 700;
		font-size: 16px;
		margin-top: 5px;
		margin-bottom: 8px;
	}
</style>
