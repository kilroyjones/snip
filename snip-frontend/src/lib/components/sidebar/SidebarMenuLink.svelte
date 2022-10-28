<script lang="ts">
	import { goto } from '$app/navigation';
	import { fade } from 'svelte/transition';
	import Fa from 'svelte-fa';

	export let icon: any;
	export let isExpanded: boolean;
	export let text: string = '';
	export let size: number = 1;
	export let link: string = '/';

	const fadeIn = {
		delay: 100,
		duration: 200
	};

	const fadeOut = {
		delay: 0,
		duration: 100
	};
</script>

<li on:click={() => goto(link)}>
	<Fa {icon} size="{size}x" fw scale={1.2} />
	{#if isExpanded}<span in:fade={fadeIn} out:fade={fadeOut}>{text}</span>{/if}
</li>

<style>
	li {
		display: flex;
		align-items: center;
		width: 200px;
		padding-left: 20px;
		height: 42px;
	}

	li:hover {
		cursor: pointer;
	}

	span {
		padding-left: 30px;
		font-family: 'Lato', Arial, sans-serif;
		font-weight: 300;
		font-size: 18px;
	}
</style>
