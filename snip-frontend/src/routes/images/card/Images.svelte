<script lang="ts">
	// import Image from './Image.svelte';
	export let images: Array<any>;

	// export let gap: number = 10;
	// export let maxColumnWidth: number = 250;
	let width: number;
	let imageColumns: Array<any> = [];
	let galleryStyle = '';

	async function update(images: Array<any>, width: number) {
		let columnCount = Math.ceil(width / 250);
		imageColumns = [];
		galleryStyle = `grid-template-columns: repeat(${columnCount}, 1fr); --gap: 10px`;
		for (let i = 0; i < images.length; i++) {
			console.log(images[i]);
			if (i < columnCount) {
				imageColumns.push([]);
			}
			console.log(imageColumns, columnCount, i % columnCount);
			imageColumns[i % columnCount].push(images[i]);
		}
	}

	$: update(images, width);
	// $: galleryStyle = `grid-template-columns: repeat(${columnCount}, 1fr); --gap: ${gap}px`;
</script>

<div id="image-gallery" bind:clientWidth={width} style={galleryStyle}>
	{#each imageColumns as column}
		<div class="column">
			{#each column as image}
				<img src="http://localhost:8080/{image.thumbnail_path}" alt="sadf" />
			{/each}
		</div>
	{/each}
	<!-- {#each images as image}
		<Image
			id={image.id}
			date={image.date}
			description={image.description}
			content={image.content}
			divWidth={width
			source={image.source}
			imagePath={image.image_path}
			thumbnailPath={image.thumbnail_path}
		/>
	{/each} -->
</div>

<style>
	#image-gallery {
		margin-top: 20px;
		width: 100%;
		display: grid;
		gap: var(--gap);
	}

	#image-gallery .column {
		display: flex;
		flex-direction: column;
	}

	#image-gallery .column * {
		width: 100%;
		margin-top: 10px;
	}

	img {
		max-width: 250px;
		border-radius: 6px;
	}
</style>
